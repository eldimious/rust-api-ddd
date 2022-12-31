use sqlx::Error as SqlxError;
use thiserror::__private::AsDynError;
use thiserror::Error as ThisError;
use warp::Rejection;

pub type Result<T> = std::result::Result<T, CustomError>;

#[derive(Clone, Debug, ThisError)]
pub enum CustomError {
    #[error("An error occurred during database interaction. {0}")]
    DatabaseError(String),
    #[error("An error occurred during http interaction. {0}")]
    HttpError(String),
}

impl From<SqlxError> for CustomError {
    fn from(sqlx_error: SqlxError) -> Self {
        match sqlx_error.as_database_error() {
            Some(db_error) => CustomError::DatabaseError(db_error.to_string()),
            None => {
                eprintln!("error {:?}", sqlx_error);
                CustomError::DatabaseError(String::from("Unrecognized database error!"))
            }
        }
    }
}

impl warp::reject::Reject for CustomError {}

pub(crate) fn custom_reject(error: impl Into<CustomError>) -> Rejection {
    warp::reject::custom(CustomError::HttpError(String::from(error.into().to_string())))
}
