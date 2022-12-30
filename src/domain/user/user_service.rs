use uuid::Uuid;
use crate::domain::user::user_model::User;
use crate::presentation::http::user::user_dto;
use crate::common::error::Error;
use crate::domain::user::user_repository::IUserRepository;
use crate::presentation::http::user::user_dto::CreateUserDto;
use crate::common::error::Result;

pub struct UserService<R>
    where
        R: IUserRepository,
{
    user_repository: R,
}

impl<R> UserService<R>
    where
        R: IUserRepository,
{
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, user: CreateUserDto) -> Result<User> {
        let user = User::try_from(user)?;
        let user = self.user_repository.create_user(&user).await?;
        Ok(user)
    }

    pub async fn list_users(&self) -> Result<Vec<User>> {
        let users =self.user_repository.list_users().await?;
        Ok(users)
    }

    pub async fn get_user(&self, user_id: &Uuid) -> Result<User> {
        let user = self.user_repository.get_user(user_id).await?;
        Ok(user)
    }
}