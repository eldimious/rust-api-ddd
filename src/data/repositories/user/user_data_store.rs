use async_trait::async_trait;
use uuid::Uuid;
use crate::common::error::{CustomError, Result};
use crate::data::infrastructure::database::DbPool;
use crate::data::repositories::user::user_entity::UserEntity;
use crate::domain::user::user_model::User;

pub struct UserDataStore {
    db_pool: DbPool,
}

impl UserDataStore {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }
}

impl UserDataStore {
    pub async fn create_user(&self, user: &User) -> Result<UserEntity> {
        let query = r#"
INSERT INTO users (first_name, last_name, email, username, password)
VALUES ($1, $2, $3, $4, $5)
RETURNING *
"#;
        let result: UserEntity = sqlx::query_as::<_, UserEntity>(query)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.password)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(result)
    }

    pub async fn list_users(&self) -> Result<Vec<UserEntity>> {
        let query = r#"SELECT * FROM users"#;
        let result: Vec<UserEntity> = sqlx::query_as::<_, UserEntity>(query)
            .fetch_all(&self.db_pool)
            .await?;

        Ok(result)
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Result<UserEntity> {
        let query = r#"SELECT * FROM users WHERE user_id = $1"#;
        let result: UserEntity = sqlx::query_as::<_, UserEntity>(query)
            .bind(&user_id)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(result)
    }
}