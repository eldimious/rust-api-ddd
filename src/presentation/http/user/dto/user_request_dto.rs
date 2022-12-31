use serde::{Deserialize};
use std::convert::TryFrom;
use crate::domain::user::user_model::User;
use thiserror::Error;
use crate::common::error::CustomError;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}


impl TryFrom<CreateUserDto> for User {
    type Error = CustomError;

    fn try_from(dto: CreateUserDto) -> Result<Self, Self::Error> {
        Ok(User {
            user_id: uuid::Uuid::nil(),
            first_name: dto.first_name,
            last_name: dto.last_name,
            email: dto.email,
            username: dto.username,
            password: dto.password
        })
    }
}