use sqlx::Error as SqlxError;
use thiserror::Error as ThisError;

#[derive(Clone, Debug, ThisError)]
pub enum Error {
    #[error("An error ocurred during database interaction. {0}")]
    DatabaseError(String),
}