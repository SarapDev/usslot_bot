use usslot_bot::{AppConfig, Result};

#[tokio::main]
async fn main () -> Result<()> {
    let _config = AppConfig::new();

    Ok(()) 
}

