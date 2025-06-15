pub use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "USSlotBot")]
#[command(version = "0.1.0")]
#[command(about = "Бот, который работает с гемблинг стикерами телеграмма")]
pub struct Args {
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,    

    #[arg(short, long, default_value = "info")]
    pub log_level: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// Запускает бота
    Bot { 
    }
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
