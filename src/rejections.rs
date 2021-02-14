use super::models::ErrReply;
use sailfish::RenderError;
use std::convert::Infallible;
use warp::body::BodyDeserializeError;
use warp::http::StatusCode;
use warp::{reject, reply, Rejection, Reply};

#[derive(Debug)]
pub(crate) struct NotFound;
impl reject::Reject for NotFound {}
impl NotFound {
    pub(crate) fn reject<T>(_: T) -> Rejection {
        reject::custom(NotFound)
    }
}

#[derive(Debug)]
pub(crate) struct InvalidPayload(String);
impl reject::Reject for InvalidPayload {}
impl InvalidPayload {
    pub(crate) fn reject<S: ToString>(message: S) -> Rejection {
        reject::custom(InvalidPayload(message.to_string()))
    }
}

#[derive(Debug)]
pub(crate) struct DbError(String);
impl reject::Reject for DbError {}
impl DbError {
    pub(crate) fn reject<S: ToString>(message: S) -> Rejection {
        reject::custom(DbError(message.to_string()))
    }
}

#[derive(Debug)]
pub(crate) struct InternalError(String);
impl reject::Reject for InternalError {}
impl InternalError {
    pub(crate) fn reject<S: ToString>(message: S) -> Rejection {
        reject::custom(InternalError(message.to_string()))
    }
}

pub(crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let mut code = StatusCode::INTERNAL_SERVER_ERROR;
    let mut message = format!("Unhandled rejection {:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "not found".to_string();
    } else if let Some(_) = err.find::<NotFound>() {
        code = StatusCode::NOT_FOUND;
        message = "not found".to_string();
    } else if let Some(invalid_note) = err.find::<InvalidPayload>() {
        code = StatusCode::BAD_REQUEST;
        message = invalid_note.0.clone();
    } else if let Some(deserialize) = err.find::<BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = deserialize.to_string();
    } else if let Some(render) = err.find::<RenderError>() {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = render.to_string();
    }

    Ok(reply::with_status(
        reply::json(&ErrReply::new(message)),
        code,
    ))
}
