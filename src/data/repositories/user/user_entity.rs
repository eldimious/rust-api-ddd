use sqlx::types::chrono;
use sqlx::FromRow;
use uuid::Uuid;
use crate::domain::user::user_model::User;

#[derive(Debug, FromRow)]
pub struct UserEntity {
    user_id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    username: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<UserEntity> for User {
    fn from(entity: UserEntity) -> Self {
        Self {
            user_id: entity.user_id,
            first_name: entity.first_name,
            last_name: entity.last_name,
            email: entity.email,
            username: entity.username,
            password: entity.password,
        }
    }
}