use super::*;

use crate::auth::{jwt_from_header, Claims, JWT_SECRET};
use crate::db::Db;
use crate::models::UserRole;
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

    Ok(decoded.claims.sub)
}
