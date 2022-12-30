use uuid::Uuid;
use crate::domain::user::model::User;
use crate::presentation::http::user::dto;
use crate::common::error::Error;
use crate::presentation::http::user::dto::CreateUserDto;

pub struct UserService;

impl UserService{
    pub fn new() -> Self {
        Self
    }

    pub async fn create_user(&self, user: CreateUserDto) -> Result<User, Error> {
        let user = User {
            user_id: Uuid::nil(),
            first_name: "Dimos".to_string(),
            last_name: "Botsaris".to_string(),
            email: "botsaris.d@gmail.com".to_string(),
            password: "test".to_string(),
            username: "eldimious".to_string(),
        };

        Ok(user)
    }

    pub async fn list_users(&self) -> Result<Vec<User>, Error> {
        let user = User {
            user_id: Uuid::nil(),
            first_name: "Dimos".to_string(),
            last_name: "Botsaris".to_string(),
            email: "botsaris.d@gmail.com".to_string(),
            password: "test".to_string(),
            username: "eldimious".to_string(),
        };
        Ok(vec![user])
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Result<User, Error> {
        let user = User {
            user_id: Uuid::nil(),
            first_name: "Dimos".to_string(),
            last_name: "Botsaris".to_string(),
            email: "botsaris.d@gmail.com".to_string(),
            password: "test".to_string(),
            username: "eldimious".to_string(),
        };
        Ok(user)
    }
}