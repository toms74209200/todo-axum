use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
    pub uid: String,
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

pub fn validate_token(secret: &[u8], token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;
    if (decoded.claims.exp >= Utc::now().timestamp() as usize)
        && (decoded.claims.iat <= Utc::now().timestamp() as usize)
    {
        Ok(decoded.claims)
    } else {
        Err(jsonwebtoken::errors::Error::from(
            jsonwebtoken::errors::ErrorKind::InvalidToken,
        ))
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    use super::*;

    mod create_token {
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

    mod validate_token {
        use super::*;

        #[test]
        fn test_validate_token_normal() {
            let secret = Uuid::new_v4().to_string();
            let uid = Uuid::new_v4().to_string();
            let mut header = Header::new(Algorithm::HS256);
            header.kid = Some("kid".to_owned());
            let jti = Uuid::new_v4().to_string();
            let claims = Claims::new(uid.to_string(), jti.clone());
            let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()));

            let claims = validate_token(&secret.as_ref(), &token.unwrap()).unwrap();

            assert_eq!(&claims.uid, &uid);
            assert_eq!(&claims.jti, &jti);
        }

        #[test]
        fn test_validate_token_invalid_secret() {
            let secret = Uuid::new_v4().to_string();
            let uid = Uuid::new_v4().to_string();
            let mut header = Header::new(Algorithm::HS256);
            header.kid = Some("kid".to_owned());
            let jti = Uuid::new_v4().to_string();
            let claims = Claims::new(uid.to_string(), jti.clone());
            let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()));

            let mut invalid_secret = secret.clone();
            invalid_secret.push_str("invalid");
            let result = validate_token(&invalid_secret.as_ref(), &token.unwrap());

            assert!(result.is_err());
        }

        #[test]
        fn test_validate_token_too_old() {
            let secret = Uuid::new_v4().to_string();
            let uid = Uuid::new_v4().to_string();
            let mut header = Header::new(Algorithm::HS256);
            header.kid = Some("kid".to_owned());
            let jti = Uuid::new_v4().to_string();
            let now = Utc::now() - Duration::from_secs(20);
            let exp = now + Duration::from_secs(10);
            let claims = Claims {
                exp: exp.timestamp() as usize,
                iat: now.timestamp() as usize,
                jti,
                uid,
            };
            let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()));

            let result = validate_token(&secret.as_ref(), &token.unwrap());

            assert!(result.is_err());
        }
    }
}
