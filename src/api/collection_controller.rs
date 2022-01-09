use crate::{
    config::db::Pool,
    constants,
    models::{collection::CollectionDTO, response::ResponseBody},
    services::{collection_service},
    utils::token_utils
};
use actix_web::{web, HttpResponse, Result};

pub async fn find_by_id(name: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match collection_service::find_by_id(&name.clone(), &pool) {
        Ok(item) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, item))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/experience
pub async fn find_by_author(author: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match collection_service::find_by_author(&author.clone(), &pool) {
        Ok(items) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, items))),
        Err(err) => Ok(err.response()),
    }
}

pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match collection_service::find_all(&pool) {
        Ok(item) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, item))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/experience
pub async fn insert(
    req: web::HttpRequest,
    mut new: web::Json<CollectionDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => {
            new.author = email;
            match collection_service::insert(new.0, &pool) {
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
    name: web::Path<String>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => match collection_service::find_by_id(&name.clone(), &pool) {
            Ok(collection) => {
                if collection.author == email {
                    match collection_service::delete(&name.clone(), &pool) {
                        Ok(()) => Ok(HttpResponse::Created()
                            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
                        Err(err) => Ok(err.response()),
                    }
                } else {
                    Ok(HttpResponse::BadRequest().body("This experience does not belong to this user"))
                }
            },
            Err(err) => Ok(err.response()),
        },
        Err(err) => Ok(HttpResponse::BadRequest().body(err))
    }
}
