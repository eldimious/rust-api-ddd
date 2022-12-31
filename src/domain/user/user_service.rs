use uuid::Uuid;
use pwhash::bcrypt;

use crate::domain::user::user_model::User;
use crate::common::error::CustomError;
use crate::domain::user::user_repository::IUserRepository;
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

    pub async fn create_user(&self, user: User) -> Result<User> {
        let mut new_user = user;
        let password_hash = bcrypt::hash(new_user.password).unwrap();
        new_user.password = password_hash;
        let user = self.user_repository.create_user(&new_user).await?;
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