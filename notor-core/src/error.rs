use thiserror::Error;
use warp::reject::Reject;

#[derive(Error, Debug)]
pub enum NotorError {
    #[error("database error - `{0}`")]
    DbError(#[from] sqlx::Error),
    #[error("UTF-8 conversion failed - `{0}`")]
    Utf8ConversionError(#[from] std::str::Utf8Error),
    #[error("invalid role `{0}`")]
    InvalidRole(String),
    #[error("timestamp was invalid")]
    InvalidTimestamp,
    #[error("user is not authorized to access this page")]
    UnauthorizedAccess,
    #[error("token verification failed - `{0}`")]
    TokenVerificationError(#[from] jsonwebtoken::errors::Error),
    #[error("no authentication header was provided")]
    AuthHeaderMissing,
    #[error("provided authentication header was invalid")]
    InvalidAuthHeader,
    #[error("provided authentication token was invalid")]
    InvalidAuthToken,
    #[error("authentication token expired")]
    AuthTokenExpired,
    #[error("provided password was invalid")]
    InvalidPassword,
    #[error("internal error - `{0}`")]
    InvalidHeaderInternalErr(#[from] warp::http::header::InvalidHeaderValue),
    #[error("internal error - `{0}`")]
    InvalidHeaderKey(#[from] warp::http::header::ToStrError),
    #[error("failed to serialize value as json `{0}`")]
    BodySerializieError(#[from] serde_json::Error),
}

impl Reject for NotorError {}
