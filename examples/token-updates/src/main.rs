use async_trait::async_trait;
use carbon_core::{
    account::{AccountMetadata, DecodedAccount},
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    metrics::MetricsCollection,
    processor::Processor,
};
use carbon_log_metrics::LogMetrics;
use carbon_token_program_decoder::{
    accounts::TokenProgramAccount, instructions::TokenProgramInstruction,
    TokenProgramDecoder, types::authority_type::AuthorityType
};
use carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient;
use solana_sdk::{pubkey, pubkey::Pubkey};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};
use tokio::sync::RwLock;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
};
use deadpool_postgres::{Config, Pool, Runtime};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use std::fs;
use env_logger::{Builder, Target};
use log::{info, warn, LevelFilter};
use std::io::Write;

pub const TOKEN_PROGRAM_ID: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub struct AppState {
    db_pool: Pool,
}

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .target(Target::Stdout)
        .init();

    info!("Starting token updates processor");
    
    dotenv::dotenv().ok();

    let mut cfg = Config::new();
    cfg.host = Some(env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string()));
    cfg.port = Some(env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string()).parse().unwrap());
    cfg.dbname = Some(env::var("POSTGRES_DB").unwrap_or_else(|_| "postgres".to_string()));
    cfg.user = Some(env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string()));
    cfg.password = Some(env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string()));

    let tls_connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .expect("Failed to build TLS connector");
    
    let connector = MakeTlsConnector::new(tls_connector);

    let pool = cfg.create_pool(Some(Runtime::Tokio1), connector)
        .expect("Failed to create connection pool");

    let app_state = Arc::new(AppState { db_pool: pool });

    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "token_program_account_filter".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![TOKEN_PROGRAM_ID.to_string().clone()],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter_token_program = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![TOKEN_PROGRAM_ID.to_string().clone()],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> =
        HashMap::new();

    transaction_filters.insert("token_program_transaction_filter".to_string(), transaction_filter_token_program);

    let yellowstone_grpc = YellowstoneGrpcGeyserClient::new(
        env::var("GEYSER_URL").unwrap_or_default(),
        env::var("X_TOKEN").ok(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Arc::new(RwLock::new(HashSet::new())),
    );

    carbon_core::pipeline::Pipeline::builder()
        .datasource(yellowstone_grpc)
        .instruction(TokenProgramDecoder, TokenProgramInstructionProcessor::new(app_state.clone()))
        .account(TokenProgramDecoder, TokenProgramAccountProcessor::new(app_state))
        .shutdown_strategy(carbon_core::pipeline::ShutdownStrategy::Immediate)
        // .metrics(Arc::new(LogMetrics::new()))
        // .metrics_flush_interval(3)
        .build()?
        .run()
        .await?;

    Ok(())
}

pub struct TokenProgramInstructionProcessor {
    app_state: Arc<AppState>,
}

impl TokenProgramInstructionProcessor {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }

    async fn update_token_data(&self, mint: &Pubkey, mint_authority: Option<Pubkey>, freeze_authority: Option<Pubkey>) -> CarbonResult<()> {
        let client = self.app_state.db_pool.get().await.unwrap();
        

    
        info!("Updating token data for mint: {}, can_mint: {:?}, can_freeze: {:?}", mint.to_string(), mint_authority.map(|pk| pk.to_string()), freeze_authority.map(|pk| pk.to_string()));
        
        let query = "
            INSERT INTO token_data_live (mint, owner, mint_authority, freeze_authority)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (mint) DO UPDATE 
            SET mint_authority = EXCLUDED.mint_authority,
                freeze_authority = EXCLUDED.freeze_authority";

        match client.execute(
            query,
            &[
                &mint.to_string(),
                &TOKEN_PROGRAM_ID.to_string(),
                &mint_authority.map(|pk| pk.to_string()),
                &freeze_authority.map(|pk| pk.to_string()),
            ],
        ).await {
            Ok(_) =>  {} //info!("Successfully updated token data for mint: {}", mint),
            Err(e) => warn!("Failed to update token data for mint {}: {}", mint, e),
        }

        Ok(())
    }
}

#[async_trait]
impl Processor for TokenProgramInstructionProcessor {
    type InputType = (InstructionMetadata, DecodedInstruction<TokenProgramInstruction>, Vec<NestedInstruction>);

    async fn process(&mut self, (metadata, instruction, _nested_instructions): Self::InputType, _metrics: Arc<MetricsCollection>) -> CarbonResult<()> {
        match instruction.data {
            TokenProgramInstruction::InitializeMint(initialize_mint) => {
                let mint = instruction.accounts[0].pubkey;
                self.update_token_data(
                    &mint,
                    Some(initialize_mint.mint_authority),
                    initialize_mint.freeze_authority,
                ).await?;
                
                // info!("Initialize mint transaction: {}", metadata.transaction_metadata.signature);
                // info!("Mint: {}", mint);
                // info!("Mint authority: {:?}", initialize_mint.mint_authority);
                // info!("Freeze authority: {:?}", initialize_mint.freeze_authority);
                // info!("Decimals: {}", initialize_mint.decimals);
            },
            TokenProgramInstruction::InitializeMint2(initialize_mint2) => {
                let mint = instruction.accounts[0].pubkey;
                self.update_token_data(
                    &mint,
                    Some(initialize_mint2.mint_authority),
                    initialize_mint2.freeze_authority,
                ).await?;
                
                // info!("Initialize mint2 transaction: {}", metadata.transaction_metadata.signature);
                // info!("Mint: {}", mint);
                // info!("Decimals: {}", initialize_mint2.decimals);
                // info!("Mint authority: {:?}", initialize_mint2.mint_authority);
                // info!("Freeze authority: {:?}", initialize_mint2.freeze_authority);
            },
            TokenProgramInstruction::SetAuthority(set_authority) => {
                let mint = instruction.accounts[0].pubkey;
                match set_authority.authority_type {
                    AuthorityType::MintTokens => {
                        self.update_token_data(&mint, set_authority.new_authority, None).await?;
                    },
                    AuthorityType::FreezeAccount => {
                        self.update_token_data(&mint, None, set_authority.new_authority).await?;
                    },
                    _ => {}
                }
                
                // info!("Set authority transaction: {}", metadata.transaction_metadata.signature);
                // info!("Mint: {}", mint);
                // info!("Authority type: {:?}", set_authority.authority_type);
                // info!("New authority: {:?}", set_authority.new_authority);
            },
            _ => {}
        }

        Ok(())
    }
}

pub struct TokenProgramAccountProcessor {
    app_state: Arc<AppState>,
}

impl TokenProgramAccountProcessor {
    pub fn new(app_state: Arc<AppState>) -> Self {
        Self { app_state }
    }
}

#[async_trait]
impl Processor for TokenProgramAccountProcessor {
    type InputType = (AccountMetadata, DecodedAccount<TokenProgramAccount>);

    async fn process(&mut self, (_, account): Self::InputType, _metrics: Arc<MetricsCollection>) -> CarbonResult<()> {
        Ok(())
    }
}

