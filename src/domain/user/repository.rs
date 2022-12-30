use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::user::model::User;
use crate::common::error::Error;

#[async_trait]
pub trait Repository {
    async fn create_user(&self, user: &User) -> Result<User, Error>;
    async fn list_users(&self) -> Result<Vec<User>, Error>;
    async fn get_user(&self, id: &Uuid) -> Result<User, Error>;
}