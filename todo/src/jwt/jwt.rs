use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    jti: String,
    uid: String,
}

impl Claims {
    pub fn new(uid: String, jti: String) -> Self {
        let now = Utc::now();
        let exp = now + Duration::from_secs(24 * 60 * 60);
        Self {
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            jti,
            uid,
        }
    }
}

pub fn create_token(
    secret: &[u8],
    uid: &String,
) -> Result<std::string::String, jsonwebtoken::errors::Error> {
    let jti = Uuid::new_v4().to_string();
    let claims = Claims::new(uid.to_string(), jti);
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some("kid".to_owned());
    encode(&header, &claims, &EncodingKey::from_secret(secret))
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    use super::*;

    #[test]
    fn test_create_token() {
        let secret = Uuid::new_v4().to_string();
        let uid = Uuid::new_v4().to_string();
        let token = create_token(&secret.as_ref(), &uid).unwrap();

        let decoded = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(&secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .unwrap();

        assert_eq!(token.split('.').count(), 3);
        assert_eq!(decoded.claims.uid, uid);
        assert!(decoded.claims.exp > Utc::now().timestamp() as usize);
        assert!(decoded.claims.iat <= Utc::now().timestamp() as usize);
        assert!(decoded.claims.jti.len() > 0);
    }
}
