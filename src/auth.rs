use crate::models::{Claims, UserRole};
use crate::Error;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use warp::http::header::{HeaderMap, HeaderValue, AUTHORIZATION};

pub const BEARER_COOKIE: &str = "Bearer";
const JWT_EXP_MIN: i64 = 2;
pub const JWT_SECRET: &[u8] = b"*-sH*y2STY4Uz^jXE8rLQ_XePB%%A?fT";
pub type Token = String;
const BEARER: &str = "Bearer ";

pub fn jwt_gen(username: String, role: &UserRole) -> Result<(Claims, Token), Error> {
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(JWT_EXP_MIN))
        .ok_or_else(|| Error::InvalidTimestamp)?
        .timestamp();

    let claim = Claims {
        sub: username,
        role: role.as_ref().to_string(),
        exp,
    };

    let header = Header::new(Algorithm::HS512);

    encode(&header, &claim, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(Error::from)
        .map(|token| (claim, token))
}

pub fn jwt_from_headers(headers: &HeaderMap<HeaderValue>) -> Result<String, Error> {
    let header = headers
        .get(AUTHORIZATION)
        .ok_or_else(|| Error::AuthHeaderMissing)?;

    let auth = std::str::from_utf8(header.as_bytes()).map_err(|_| Error::InvalidAuthHeader)?;

    if !auth.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeader);
    }

    Ok(auth.trim_start_matches(BEARER).to_string())
}
