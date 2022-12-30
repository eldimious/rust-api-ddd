use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use warp::reply::json;
use warp::{reject, Rejection, Reply};
use crate::domain::user::user_service::UserService;
use crate::presentation::http::user::user_dto::CreateUserDto;
use crate::UserRepository;

#[derive(Deserialize)]
pub struct CreateUserPayload {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

pub async fn list_users(
    user_service: Arc<UserService<UserRepository>>,
) -> Result<impl Reply, Rejection> {
    match user_service.list_users().await {
        Ok(users) => Ok(json(
            &users
                .into_iter()
                .map(|u| UserResponse {
                    user_id: u.user_id,
                    first_name: u.first_name,
                    last_name: u.last_name,
                    email: u.email,
                    username: u.username,
                })
                .collect::<Vec<UserResponse>>(),
        )),
        Err(e) => {
            Err(reject())
        }
    }
}

pub async fn create_user(
    user_service: Arc<UserService<UserRepository>>,
    body: CreateUserPayload,
) -> Result<impl Reply, Rejection> {
    match user_service
        .create_user(CreateUserDto {
            first_name: body.first_name,
            last_name: body.last_name,
            email: body.email,
            username: body.username,
            password: body.password,
        })
        .await
    {
        Ok(created_user) => Ok(json(&UserResponse {
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