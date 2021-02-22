use sailfish::RenderError;
use std::convert::Infallible;
use thiserror::Error;
use warp::body::BodyDeserializeError;
use warp::http::{StatusCode, Uri};
use warp::{
    reject::{InvalidHeader, Reject},
    reply, Rejection, Reply,
};

use crate::models::ErrReply;
use crate::web::{html_from, Login, INDEX_SCRIPT, INDEX_STYLE};

type Response = Result<warp::reply::Response, Infallible>;

fn redirect_login() -> impl Reply {
    warp::redirect::temporary(Uri::from_static("/web/login"))
}

fn render_login_err(message: &str) -> impl Reply {
    let view = Login::new(message);

    if let Ok(html) = html_from(view, "Login".to_string(), INDEX_SCRIPT, INDEX_STYLE) {
        reply::html(html).into_response()
    } else {
        reply::html(message.to_string()).into_response()
    }
}

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
    #[error("user is not authorized to access this page")]
    UnauthorizedAccess,
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
    #[error("internal error - `{0}`")]
    InvalidHeaderInternalErr(#[from] warp::http::header::InvalidHeaderValue),
    #[error("internal error - `{0}`")]
    InvalidHeaderKey(#[from] warp::http::header::ToStrError),
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
            TokenCreationError(_)
            | Utf8ConversionError(_)
            | RenderError(_)
            | InvalidTimestamp
            | InvalidHeaderKey(_)
            | InvalidHeaderInternalErr(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
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
    } else if let Some(err) = err.find::<InvalidHeader>() {
        match err.name() {
            "Cookie" => return Ok(redirect_login().into_response()) as Response,
            _ => {
                code = StatusCode::BAD_REQUEST;
                message = err.to_string();
            }
        }
    } else if let Some(err) = err.find::<WebError>() {
        return handle_web_rejection(err).await.map(|v| v.into_response());
    }

    Ok(reply::with_status(reply::json(&ErrReply::new(message)), code).into_response()) as Response
}

pub(crate) async fn handle_web_rejection(err: &WebError) -> Result<impl Reply, Infallible> {
    use sqlx::Error::*;
    use RejectError::*;

    let (_, message) = match err {
        WebError::Inner(err) => match err {
            DbError(err) => match err {
                RowNotFound => (StatusCode::NOT_FOUND, "not found".into()),
                // #TODO: handle all
                _ => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            },
            InvalidRole(_) => (StatusCode::BAD_REQUEST, err.to_string()),
            TokenCreationError(_)
            | Utf8ConversionError(_)
            | RenderError(_)
            | InvalidTimestamp
            | InvalidHeaderKey(_)
            | InvalidHeaderInternalErr(_) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            InvalidAuthHeader | InvalidAuthToken | InvalidPassword => {
                (StatusCode::FORBIDDEN, err.to_string())
            }
            AuthTokenExpired | AuthHeaderMissing => {
                return Ok(render_login_err("Authentication token expired").into_response())
                    as Response;
            }
            UnauthorizedAccess => (StatusCode::UNAUTHORIZED, err.to_string()),
        },
    };

    Ok(render_login_err(&message).into_response()) as Response
}
