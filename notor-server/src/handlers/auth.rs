use warp::{reject, Rejection, Reply};

use crate::db::Db;

use crate::auth::{jwt_from_headers, jwt_gen, BEARER_COOKIE, JWT_EXP_MIN, JWT_SECRET};
use crate::models::{delete_claims, load_claims, load_claims_if_exists, load_user, save_claims};
use crate::Error;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use notor_core::models::{Claims, JsonAuth, UserRole};
use warp::http::{
    header::{HeaderMap, HeaderValue},
    Response,
};

pub async fn authorize_headers(
    (role, db, headers): (UserRole, Db, HeaderMap<HeaderValue>),
) -> Result<String, Rejection> {
    let token = jwt_from_headers(&headers).map_err(reject::custom)?;

    authorize_token((role, db, token)).await
}

pub async fn authorize_token(
    (role, db, token): (UserRole, Db, String),
) -> Result<String, Rejection> {
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(Error::from)
    .map_err(reject::custom)?;

    if role != decoded.claims.role.parse()? {}

    let claim = load_claims(&decoded.claims.sub, &db)
        .await
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

pub(crate) async fn handle_login(auth: JsonAuth, conn: Db) -> Result<impl Reply, Rejection> {
    let user = load_user(&auth.username, &conn)
        .await
        .map_err(reject::custom)?;

    if user.pass != auth.pass {
        return Err(reject::custom(Error::InvalidPassword));
    }

    if let Some(_) = load_claims_if_exists(&user.username, &conn).await {
        delete_claims(&user.username, &conn)
            .await
            .map_err(reject::custom)?;
    }

    let (claim, token) = jwt_gen(user.username, &user.role)?;

    save_claims(&claim, &conn).await.map_err(reject::custom)?;

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
