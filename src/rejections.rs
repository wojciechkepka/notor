use super::models::ErrReply;
use sailfish::RenderError;
use std::convert::Infallible;
use thiserror::Error;
use warp::body::BodyDeserializeError;
use warp::http::StatusCode;
use warp::{reject, reply, Rejection, Reply};

#[derive(Error, Debug)]
pub enum RejectError {
    #[error("database error - `{0}`")]
    DbError(#[from] sqlx::Error),
    #[error("rendering template failed - `{0}`")]
    RenderError(#[from] RenderError),
    #[error("UTF-8 conversion failed - `{0}`")]
    Utf8ConversionError(#[from] std::str::Utf8Error),
    #[error("invalid role `{0}`")]
    InvalidRole(String),
    #[error("timestamp was invalid")]
    InvalidTimestamp,
    #[error("creating auth token failed - `{0}`")]
    TokenCreationError(#[from] jsonwebtoken::errors::Error),
    #[error("no authentication header was provided")]
    AuthHeaderMissing,
    #[error("provided authentication header was invalid")]
    InvalidAuthHeader,
    #[error("user has no perrmision to access this secition")]
    UnauthorizedAccess,
    #[error("provided authentication token was invalid")]
    InvalidAuthToken,
    #[error("authentication token expired")]
    AuthTokenExpired,
    #[error("provided password was invalid")]
    InvalidPassword,
}
impl reject::Reject for RejectError {}

impl RejectError {
    fn reply(&self) -> (StatusCode, String) {
        use sqlx::Error::*;
        use RejectError::*;

        match self {
            DbError(err) => match err {
                RowNotFound => (StatusCode::NOT_FOUND, "not found".into()),
                // #TODO: handle all
                err => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            },
            InvalidRole(role) => (StatusCode::BAD_REQUEST, self.to_string()),
            TokenCreationError(_) | Utf8ConversionError(_) | RenderError(_) | InvalidTimestamp => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AuthHeaderMissing | InvalidAuthHeader | InvalidAuthToken | AuthTokenExpired
            | InvalidPassword => (StatusCode::FORBIDDEN, self.to_string()),
            UnauthorizedAccess => (StatusCode::UNAUTHORIZED, self.to_string()),
        }
    }
}

pub(crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let mut code = StatusCode::INTERNAL_SERVER_ERROR;
    let mut message = format!("Unhandled rejection {:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "not found".to_string();
    } else if let Some(deserialize) = err.find::<BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = deserialize.to_string();
    } else if let Some(err) = err.find::<RejectError>() {
        let (c, m) = err.reply(); // issue #71126 destructuring_assignment
        code = c;
        message = m;
    }

    Ok(reply::with_status(
        reply::json(&ErrReply::new(message)),
        code,
    ))
}
