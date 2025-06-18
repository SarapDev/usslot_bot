use mongodb::bson::oid::ObjectId;
use mongodb::bson::doc;
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use serde::{Deserialize, Serialize};
use crate::telergam::types::User as TelegramUser;
use crate::{DatabaseConnection, Result};

pub struct UserRepository {
    collection: mongodb::Collection<User>   
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub telergam_id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub balance: i64,
}

impl From<&User> for TelegramUser { 
    fn from(t: &User) -> Self {
        TelegramUser {
            id: t.telergam_id,
            first_name: t.first_name.clone(),
            last_name: t.last_name.clone(),
            username: t.username.clone(),
            is_bot: t.is_bot,
            language_code: t.language_code.clone(),
        }
    }
}

impl From<&TelegramUser> for User {
    fn from(t: &TelegramUser) -> Self {
        User {
            id: ObjectId::new(),
            telergam_id: t.id,
            first_name: t.first_name.clone(),
            last_name: t.last_name.clone(),
            username: t.username.clone(),
            is_bot: t.is_bot,
            language_code: t.language_code.clone(),
            balance: 0,
        }
    }
}

impl UserRepository {
    pub fn new(db: &DatabaseConnection) -> Self {
        Self {
            collection: db.get_collection("users"),
        }
    }
    
    /// Create new user instance with default balance
    pub async fn create(&self, user: &TelegramUser) -> Result<User> {
        let mut user = User::from(user);
        user.balance = 200;
        self.collection.insert_one(&user, None).await?;
        
        Ok(user)
    }
 
    /// Update user instance - FIXED: use telergam_id instead of id
    pub async fn update(&self, user: &User) -> Result<Option<User>> {
        self.collection
            .replace_one(doc! { "telergam_id": user.telergam_id }, user, None)
            .await?;
        Ok(Some(user.clone()))
    }
    
    /// Find user by telegram ID 
    pub async fn get_by_id(&self, id: i64) -> Result<Option<User>> {
        let user = self.collection
            .find_one(doc! { "telergam_id": id }, None)
            .await?;
        Ok(user)
    }
    
    /// Atomically update user balance - prevents race conditions
    pub async fn atomic_balance_update(&self, telegram_id: i64, balance_change: i64) -> Result<Option<User>> {
        let filter = doc! { "telergam_id": telegram_id };
        let update = doc! { "$inc": { "balance": balance_change } };
        
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();
            
        let updated_user = self.collection
            .find_one_and_update(filter, update, options)
            .await?;
            
        Ok(updated_user)
    }
    
    /// Get or create user - simple approach
    pub async fn get_or_create(&self, telegram_user: &TelegramUser) -> Result<User> {
        // First try to get existing user
        if let Some(existing_user) = self.get_by_id(telegram_user.id).await? {
            return Ok(existing_user);
        }
        
        // Try to create new user
        self.create(telegram_user).await
    }
    
    /// Atomic bet operation - deduct bet and add winnings in one operation
    pub async fn process_bet(&self, telegram_id: i64, bet_amount: i64, win_amount: i64) -> Result<Option<User>> {
        let net_change = win_amount - bet_amount;
        
        // Only proceed if user has enough balance
        let balance_check = doc! { 
            "telergam_id": telegram_id,
            "balance": { "$gte": bet_amount }
        };
        
        let update = doc! { "$inc": { "balance": net_change } };
        
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();
            
        let updated_user = self.collection
            .find_one_and_update(balance_check, update, options)
            .await?;
            
        Ok(updated_user)
    }
}
