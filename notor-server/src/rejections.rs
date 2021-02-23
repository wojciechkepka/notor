use std::convert::Infallible;
use warp::body::BodyDeserializeError;
use warp::http::StatusCode;
use warp::{reject::InvalidHeader, reply, Rejection, Reply};

use crate::Error;
use notor_core::models::ErrReply;

type Response = Result<warp::reply::Response, Infallible>;

pub(crate) async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    use sqlx::Error::*;
    use Error::*;

    let mut code = StatusCode::INTERNAL_SERVER_ERROR;
    let mut message = format!("Unhandled rejection {:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "not found".to_string();
    } else if let Some(deserialize) = err.find::<BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = deserialize.to_string();
    } else if let Some(err) = err.find::<Error>() {
        let (c, m) = match err {
            DbError(inner) => match inner {
                RowNotFound => (StatusCode::NOT_FOUND, "not found".into()),
                // #TODO: handle all
                _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            },
            InvalidRole(_) => (StatusCode::BAD_REQUEST, err.to_string()),
            TokenVerificationError(_)
            | Utf8ConversionError(_)
            | InvalidTimestamp
            | InvalidHeaderKey(_)
            | BodySerializieError(_)
            | InvalidHeaderInternalErr(_) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AuthHeaderMissing | InvalidAuthHeader | InvalidAuthToken | AuthTokenExpired
            | InvalidPassword => (StatusCode::FORBIDDEN, err.to_string()),
            UnauthorizedAccess => (StatusCode::UNAUTHORIZED, err.to_string()),
        };
        code = c;
        message = m;
    } else if let Some(err) = err.find::<InvalidHeader>() {
        code = StatusCode::BAD_REQUEST;
        message = err.to_string();
    }

    Ok(reply::with_status(reply::json(&ErrReply::new(message)), code).into_response()) as Response
}
