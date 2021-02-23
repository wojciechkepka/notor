use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use warp::{
    http::{
        header::{HeaderMap, HeaderValue},
        Response,
    },
    reject, Rejection, Reply,
};

use crate::auth::{jwt_from_headers, jwt_gen, BEARER_COOKIE, JWT_EXP_MIN, JWT_SECRET};
use crate::db::Db;
use crate::models::{delete_claims, load_claims, load_claims_if_exists, load_user, save_claims};
use crate::Error;
use notor_core::models::{Claims, JsonAuth, JsonToken, UserRole};

pub async fn authorize_headers(
    (role, db, headers): (UserRole, Db, HeaderMap<HeaderValue>),
) -> Result<String, Rejection> {
    let token = jwt_from_headers(&headers)?;

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
    .map_err(Error::from)?;

    if role != decoded.claims.role.parse()? {}

    let claim = load_claims(&decoded.claims.sub, &db).await?;

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
    let user = load_user(&auth.username, &conn).await?;

    if user.pass != auth.pass {
        return Err(reject::custom(Error::InvalidPassword));
    }

    if let Some(_) = load_claims_if_exists(&user.username, &conn).await {
        delete_claims(&user.username, &conn).await?;
    }

    let (claim, token) = jwt_gen(user.username, &user.role)?;

    save_claims(&claim, &conn).await?;

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
        .body(serde_json::to_string(&JsonToken::new(token)).map_err(Error::from)?)
        .into_response())
}
