use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::user::user_model::User;
use crate::common::error::{Result, Error};

#[async_trait]
pub trait IUserRepository {
    async fn create_user(&self, user: &User) -> Result<User>;
    async fn list_users(&self) -> Result<Vec<User>>;
    async fn get_user(&self, id: &Uuid) -> Result<User>;
}