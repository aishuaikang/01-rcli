use std::time::Duration;

use jsonwebtoken::{
    get_current_timestamp, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String, // Optional. Audience
    pub exp: u64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub sub: String, // Optional. Subject (whom token refers to)
}

pub fn process_jwt_sign(sub: &str, aud: &str, exp: Duration, key: &str) -> anyhow::Result<String> {
    let claims = Claims {
        aud: aud.to_string(),
        exp: get_current_timestamp() + exp.as_secs(),
        sub: sub.to_string(),
    };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )?;
    Ok(token)
}

pub fn process_jwt_verify(token: &str, key: &str) -> anyhow::Result<TokenData<Claims>> {
    let mut validation = Validation::default();
    validation.validate_aud = false;
    let token = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation,
    )?;
    Ok(token)
}
