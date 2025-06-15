use usslot_bot::{AppConfig, Args, Commands, Result, UpdateHandler};
use log::info;

#[tokio::main]
async fn main () -> Result<()> {
    let args = Args::parse();
    let config = AppConfig::new(&args).map_err(|e| {
        eprintln!("Failed to load configuration: {}", e);
        e
    })?;

    init_logger(&config.logger.log_level)?;

    match &args.command {
        Commands::Bot => {
            info!("Starting bot...");
            let bot = UpdateHandler::new(config.bot.token.clone()); 

            bot.run().await;
        }
    }

    Ok(()) 
}

fn init_logger(log_level: &str) -> Result<()> {
    let level = match log_level.to_lowercase().as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => {
            eprintln!("Invalid log level: {}. Using 'info' as default.", log_level);
            log::LevelFilter::Info
        }
    };

    env_logger::Builder::from_default_env()
        .filter_level(level)
        .init();

    Ok(())
}

