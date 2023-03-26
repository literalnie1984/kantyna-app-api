use actix_web::http::header;
use actix_web::FromRequest;
use chrono::Utc;
use jsonwebtoken::errors::Error as JwtError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;

//change @ Release
const JWT_SECRET: &[u8] =
    "gioryegioergb389458y85w4huuhierghlgrezhlgh89y5w48954w4w5huoiyh".as_bytes();

#[derive(Apiv2Schema)]
pub struct AuthUser {
    pub id: i32,
}

impl FromRequest for AuthUser {
    type Error = ServiceError;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        fn return_func(err: ServiceError) -> std::future::Ready<Result<AuthUser, ServiceError>> {
            std::future::ready(Err(err))
        }
        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(l) => l,
            None => {
                return return_func(ServiceError::BadRequest(
                    "No authorization header".to_string(),
                ))
            }
        };

        let auth_header = match auth_header.to_str() {
            Ok(l) => l,
            Err(_) => {
                return return_func(ServiceError::BadRequest(
                    "Ivalid authorization header".to_string(),
                ))
            }
        };

        let token = match auth_header.split("Bearer ").nth(1) {
            Some(l) => l,
            None => {
                return return_func(ServiceError::BadRequest(
                    "Ivalid authorization header".to_string(),
                ))
            }
        };

        let user_id = match decode_jwt_token(token.to_string()) {
            Ok(l) => l,
            Err(_) => return return_func(ServiceError::JWTInvalidToken),
        };

        std::future::ready(Ok(AuthUser { id: user_id }))
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(uid: i32) -> Result<String, JwtError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

fn decode_jwt_token(token: String) -> Result<i32, ServiceError> {
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|_| ServiceError::JWTInvalidToken)?;

    decoded
        .claims
        .sub
        .parse::<i32>()
        .map_err(|_| ServiceError::JWTInvalidToken)
}
