use serde::{Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserResponseDto {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}