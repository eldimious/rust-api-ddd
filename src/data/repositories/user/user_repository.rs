use async_trait::async_trait;
use crate::common::error::CustomError;
use crate::data::repositories::user::user_data_store::UserDataStore;
use crate::data::repositories::user::user_entity::UserEntity;
use crate::domain::user::user_model::User;
use crate::domain::user::user_repository::IUserRepository;

pub struct UserRepository {
    data_store: UserDataStore,
}

impl UserRepository {
    pub fn new(data_store: UserDataStore) -> Self {
        Self { data_store }
    }
}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn create_user(&self, user: &User) -> Result<User, CustomError> {
        let result: UserEntity = self
            .data_store
            .create_user(user)
            .await?;

        Ok(User::from(result))
    }

    async fn list_users(&self) -> Result<Vec<User>, CustomError> {
        let result: Vec<UserEntity> = self
            .data_store
            .list_users()
            .await?;

        Ok(result.into_iter().map(User::from).collect())
    }

    async fn get_user(&self, user_id: &uuid::Uuid) -> Result<User, CustomError> {
        let result: UserEntity = self
            .data_store
            .get_user(user_id)
            .await?;

        Ok(User::from(result))
    }
}