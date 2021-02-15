use super::models::ErrReply;
use sailfish::RenderError;
use std::convert::Infallible;
use thiserror::Error;
use warp::body::BodyDeserializeError;
use warp::http::StatusCode;
use warp::{reject, reply, Rejection, Reply};

#[derive(Error, Debug)]
pub(crate) enum RejectError {
    #[error("database error")]
    DbError(#[from] sqlx::Error),
    #[error("rendering template failed")]
    RenderError(#[from] RenderError),
    #[error("UTF-8 conversion failed")]
    Utf8ConversionError(#[from] std::str::Utf8Error),
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
                err => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            },
            RenderError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            Utf8ConversionError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
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
