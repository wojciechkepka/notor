use super::*;

use crate::auth::{jwt_from_headers, jwt_gen, BEARER_COOKIE, JWT_EXP_MIN, JWT_SECRET};
use crate::db::Db;
use crate::models::{Claims, JsonAuth, User, UserRole};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use warp::http::{
    header::{HeaderMap, HeaderValue},
    Response,
};

pub async fn authorize(
    (role, db, headers): (UserRole, Db, HeaderMap<HeaderValue>),
) -> Result<String, Rejection> {
    let token = jwt_from_headers(&headers)
        .map_err(Error::from)
        .map_err(reject::custom)?;

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(Error::from)
    .map_err(reject::custom)?;

    if role != decoded.claims.role.parse()? {}

    let claim = Claims::load(&decoded.claims.sub, &db)
        .await
        .map_err(Error::from)
        .map_err(reject::custom)?;

    if claim != decoded.claims {
        // claim doesn't match the database entry
        return Err(reject::custom(Error::InvalidAuthToken));
    }

    if decoded.claims.is_expired() {
        // token expired
        return Err(reject::custom(Error::AuthTokenExpired));
    }

    Ok(decoded.claims.sub)
}

pub async fn authorize_web((role, db, token): (UserRole, Db, String)) -> Result<String, Rejection> {
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(Error::from)
    .map_err(WebError::from)
    .map_err(reject::custom)?;

    if role != decoded.claims.role.parse()? {}

    let claim = Claims::load(&decoded.claims.sub, &db)
        .await
        .map_err(Error::from)
        .map_err(WebError::from)
        .map_err(reject::custom)?;

    if claim != decoded.claims {
        // claim doesn't match the database entry
        return Err(reject::custom(WebError::from(Error::InvalidAuthToken)));
    }

    if decoded.claims.is_expired() {
        // token expired
        return Err(reject::custom(WebError::from(Error::AuthTokenExpired)));
    }

    Ok(decoded.claims.sub)
}

pub(crate) async fn handle_login(auth: JsonAuth, conn: Db) -> Result<impl Reply, Rejection> {
    let user = User::load(&auth.username, &conn)
        .await
        .map_err(Error::from)
        .map_err(reject::custom)?;

    if user.pass != auth.pass {
        return Err(reject::custom(Error::InvalidPassword));
    }

    if let Some(_) = Claims::load_if_exists(&user.username, &conn).await {
        Claims::delete(&user.username, &conn)
            .await
            .map_err(Error::from)
            .map_err(reject::custom)?;
    }

    let (claim, token) = jwt_gen(user.username, &user.role)?;

    Claims::save(&claim, &conn)
        .await
        .map_err(Error::from)
        .map_err(reject::custom)?;

    Ok(Response::builder()
        .header(
            "Set-Cookie",
            &format!(
                "{}={}; max-age={}; SameSite=Strict",
                BEARER_COOKIE,
                token,
                JWT_EXP_MIN * 60
            ),
        )
        .header(
            "Set-Cookie",
            &format!(
                "Username={}; max-age={}; SameSite=Strict",
                auth.username,
                JWT_EXP_MIN * 60
            ),
        )
        .body(token)
        .into_response())
}
