use super::*;

use crate::auth::{jwt_from_header, jwt_gen, JWT_SECRET};
use crate::db::Db;
use crate::models::{Claims, JsonAuth, User, UserRole};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use warp::http::{HeaderMap, HeaderValue};

pub async fn authorize(
    (role, db, headers): (UserRole, Db, HeaderMap<HeaderValue>),
) -> Result<String, Rejection> {
    let token = jwt_from_header(&headers).map_err(reject::custom)?;

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

pub(crate) async fn handle_login(auth: JsonAuth, conn: Db) -> Result<impl Reply, Rejection> {
    let user = User::load(&auth.username, &conn)
        .await
        .map_err(Error::from)
        .map_err(reject::custom)?;

    if user.pass != auth.pass {
        return Err(reject::custom(Error::InvalidPassword));
    }

    if let Some(claims) = Claims::load_if_exists(&user.username, &conn).await {
        Claims::delete(&user.username, &conn)
            .await
            .map_err(Error::from)
            .map_err(reject::custom)?;
    }

    Ok(jwt_gen(user.username, &user.role)?)
}
