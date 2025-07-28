use std::sync::Arc;

use crate::{repository::user::UserRepository, Result};

pub struct BalanceService {
    repository: Arc<UserRepository>,
}

impl BalanceService {
    pub fn new(repository: Arc<UserRepository>) -> Self {
        Self {
            repository,
        }
    }

    pub async fn handle(&self, user_id: i64) -> Result<Option<String>> {
        let balance = match self.repository.get_by_id(user_id).await? {
            Some(user) => user.balance,
            None => 0,
        };

        Ok(Some(format!("Твой баласн: {}", balance)))
    }
}
