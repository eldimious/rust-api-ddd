use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use warp::reply::json;
use warp::{Error, reject, Rejection, Reply};
use crate::domain::user::user_model::User;
use crate::domain::user::user_service::UserService;
use crate::presentation::http::user::dto::user_request_dto::CreateUserDto;
use crate::presentation::http::user::dto::user_response_dto::UserResponseDto;
use crate::UserRepository;

pub async fn list_users(
    user_service: Arc<UserService<UserRepository>>,
) -> Result<impl Reply, Rejection> {
    match user_service.list_users().await {
        Ok(users) => Ok(json(
            &users
                .into_iter()
                .map(|u| UserResponseDto {
                    user_id: u.user_id,
                    first_name: u.first_name,
                    last_name: u.last_name,
                    email: u.email,
                    username: u.username,
                })
                .collect::<Vec<UserResponseDto>>(),
        )),
        Err(e) => {
            Err(reject())
        }
    }
}

pub async fn create_user(
    user_service: Arc<UserService<UserRepository>>,
    body: CreateUserDto,
) -> Result<impl Reply, Rejection> {
    let new_user = User::try_from(body).map_err(reject::custom)?;
    match user_service
        .create_user(new_user)
        .await
    {
        Ok(created_user) => Ok(json(&UserResponseDto {
            user_id: created_user.user_id,
            first_name: created_user.first_name,
            last_name: created_user.last_name,
            email: created_user.email,
            username: created_user.username,
        })),
        Err(e) => {
            Err(reject())
        }
    }
}

pub async fn get_user(
    user_service: Arc<UserService<UserRepository>>,
    id: Uuid,
) -> Result<impl Reply, Rejection> {
    match user_service
        .get_user(&id)
        .await
    {
        Ok(user) => Ok(json(&UserResponseDto {
            user_id: user.user_id,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            username: user.username,
        })),
        Err(e) => {
            Err(reject())
        }
    }
}