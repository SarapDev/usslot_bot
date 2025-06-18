use mongodb::{Client, Database};
use log::{info, error};
use crate::config::config::Database as DatabaseConfig;
use crate::errors::bot_error::BotError;
use crate::Result;

#[derive(Debug)]
pub struct DatabaseConnection {
    pub client: Client,
    pub database: Database,
}

impl DatabaseConnection {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        info!("Connecting to MongoDB at: {}", config.url);
        
        let client = Client::with_uri_str(&config.url).await?;
        
        // Test the connection
        match client.list_database_names(None, None).await {
            Ok(_) => info!("Successfully connected to MongoDB"),
            Err(e) => {
                error!("Failed to connect to MongoDB: {}", e);
                return Err(BotError::Database(e));
            }
        }
        
        let database = client.database(&config.name);
        info!("Connected to database: {}", config.name);
        
        Ok(Self { client, database })
    }

    pub fn get_collection<T>(&self, name: &str) -> mongodb::Collection<T> {
        self.database.collection(name)
    }

    pub async fn ping(&self) -> Result<()> {
        use mongodb::bson::doc;
        
        self.database
            .run_command(doc! { "ping": 1 }, None)
            .await?;
            
        Ok(())
    }
}
