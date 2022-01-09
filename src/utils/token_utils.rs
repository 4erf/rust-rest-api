use crate::{
    config::db::Pool,
    constants,
    models::{
        user::User,
        user_token::{UserToken, KEY},
    },
    utils::token_utils
};
use actix_web::web;
use jsonwebtoken::{DecodingKey, TokenData, Validation};

pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

pub fn verify_token(
    token_data: &TokenData<UserToken>,
    pool: &web::Data<Pool>,
) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, &pool.get().unwrap()) {
        Ok(token_data.claims.email.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}

pub fn get_token_email(req: web::HttpRequest) -> Result<String, String> {
    match req.headers().get(constants::AUTHORIZATION) {
        Some(auth_header) => match auth_header.to_str() {
            Ok(auth_str) => {
                if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer")
                {
                    let token = auth_str[6..auth_str.len()].trim();
                    match token_utils::decode_token(token.to_string())
                    {
                        Ok(token_data) => Ok(token_data.claims.email),
                        Err(_) => Err("Cannot decode auth token".to_owned())
                    }
                } else {
                    Err("Invalid auth header".to_owned())
                }
            },
            Err(_) => Err("Invalid auth header".to_owned())
        },
        None => Err("No auth header".to_owned())
    }
}