use sailfish::RenderError;
use std::convert::Infallible;
use thiserror::Error;
use warp::body::BodyDeserializeError;
use warp::http::StatusCode;
use warp::{reject::Reject, reply, Rejection, Reply};

use crate::models::ErrReply;
use crate::web::{html_from, Login, INDEX_SCRIPT, INDEX_STYLE};

type Response = Result<warp::reply::Response, Infallible>;

#[derive(Error, Debug)]
pub enum WebError {
    #[error("`{0}`")]
    Inner(#[from] RejectError),
}

impl Reject for WebError {}

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
    #[error("provided authentication token was invalid")]
    InvalidAuthToken,
    #[error("authentication token expired")]
    AuthTokenExpired,
    #[error("provided password was invalid")]
    InvalidPassword,
}
impl Reject for RejectError {}

impl RejectError {
    fn reply(&self) -> (StatusCode, String) {
        use sqlx::Error::*;
        use RejectError::*;

        match self {
            DbError(err) => match err {
                RowNotFound => (StatusCode::NOT_FOUND, "not found".into()),
                // #TODO: handle all
                _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            },
            InvalidRole(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            TokenCreationError(_) | Utf8ConversionError(_) | RenderError(_) | InvalidTimestamp => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AuthHeaderMissing | InvalidAuthHeader | InvalidAuthToken | AuthTokenExpired
            | InvalidPassword => (StatusCode::FORBIDDEN, self.to_string()),
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
    } else if let Some(err) = err.find::<WebError>() {
        return handle_web_rejection(err).await.map(|v| v.into_response());
    }

    Ok(reply::with_status(reply::json(&ErrReply::new(message)), code).into_response()) as Response
}

pub(crate) async fn handle_web_rejection(err: &WebError) -> Result<impl Reply, Infallible> {
    let message = match err {
        WebError::Inner(err) => err.reply().1,
    };

    let view = Login::new(&message);

    if let Ok(html) = html_from(view, "Login".to_string(), INDEX_SCRIPT, INDEX_STYLE) {
        Ok(reply::html(html).into_response()) as Response
    } else {
        Ok(reply::html(message).into_response()) as Response
    }
}
