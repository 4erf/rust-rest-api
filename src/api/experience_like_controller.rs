use crate::{
    config::db::Pool,
    constants,
    models::{experience_like::{ExperienceLike, TopUserRowDTO}, response::ResponseBody},
    services::{experience_like_service},
    utils::token_utils
};
use actix_web::{web, HttpResponse, Result};

// GET api/experience
pub async fn count_all_by_experience(web::Path((country, city)): web::Path<(String, String)>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match experience_like_service::count_all_by_experience(&country, &city, &pool) {
        Ok(count) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, count))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn get_top_users(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match experience_like_service::get_top_users(&pool) {
        Ok(top) => Ok(
            HttpResponse::Ok().json(
                ResponseBody::new(
                    constants::MESSAGE_OK,
                    top.into_iter().map(|row| row.into()).collect::<Vec<TopUserRowDTO>>()
                )
            )
        ),
        Err(err) => Ok(err.response()),
    }
}

pub async fn get_user_likes(email: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match experience_like_service::get_user_likes(&email, &pool) {
        Ok(count) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, count))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn has_user_liked_experience(web::Path((email, country, city)): web::Path<(String, String, String)>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match experience_like_service::has_user_liked_experience(&email, &country, &city, &pool) {
        Ok(count) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, count))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/experience
pub async fn insert(
    req: web::HttpRequest,
    params: web::Path<(String, String)>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => {
            let new = ExperienceLike {
                user: email,
                country: params.clone().0,
                city: params.clone().1,
            };
            match experience_like_service::insert(new, &pool) {
                Ok(()) => Ok(HttpResponse::Created()
                    .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
                Err(err) => Ok(err.response()),
            }
        },
        Err(err) => Ok(HttpResponse::BadRequest().body(err))
    }
}

// DELETE api/experience/{country}/{city}
pub async fn delete(
    req: web::HttpRequest,
    params: web::Path<(String, String)>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => {
            let pk = (email, params.clone().0, params.clone().1);
            match experience_like_service::delete(&pk, &pool) {
                Ok(()) => Ok(HttpResponse::Created()
                    .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
                Err(err) => Ok(err.response()),
            }
        },
        Err(err) => Ok(HttpResponse::BadRequest().body(err))
    }
}
