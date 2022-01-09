use crate::{
    config::db::Pool,
    constants,
    models::{collection_experience::CollectionExperience, response::ResponseBody},
    services::{collection_experience_service, collection_service, experience_service},
    utils::token_utils
};
use actix_web::{web, HttpResponse, Result};

// GET api/experience
pub async fn find_by_collection(author: web::Path<String>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match collection_experience_service::find_by_collection(&author.clone(), &pool) {
        Ok(items) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, items))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/experience
pub async fn insert(
    req: web::HttpRequest,
    new: web::Json<CollectionExperience>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => match collection_service::find_by_id(&new.name.clone(), &pool) {
            Ok(collection) => match experience_service::find_by_id((new.country.clone(), new.city.clone()), &pool) {
                Ok(_) => {
                    if collection.author == email {
                        match collection_experience_service::insert(new.0, &pool) {
                            Ok(()) => Ok(HttpResponse::Created()
                                .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
                            Err(err) => Ok(err.response()),
                        }
                    } else {
                        Ok(HttpResponse::BadRequest().body("This collection does not belong to this user"))
                    }
                },
                Err(err) => Ok(err.response()),
            },
            Err(err) => Ok(err.response()),
        },
        Err(err) => Ok(HttpResponse::BadRequest().body(err))
    }
}

// DELETE api/experience/{country}/{city}
pub async fn delete(
    req: web::HttpRequest,
    web::Path((name, country, city)): web::Path<(String, String, String)>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => match collection_service::find_by_id(&name.clone(), &pool) {
            Ok(collection) => match experience_service::find_by_id((country.clone(), city.clone()), &pool) {
                Ok(_) => {
                    if collection.author == email {
                        match collection_experience_service::delete(&(name, country, city), &pool) {
                            Ok(()) => Ok(HttpResponse::Created()
                                .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
                            Err(err) => Ok(err.response()),
                        }
                    } else {
                        Ok(HttpResponse::BadRequest().body("This collection does not belong to this user"))
                    }
                },
                Err(err) => Ok(err.response()),
            },
            Err(err) => Ok(err.response()),
        },
        Err(err) => Ok(HttpResponse::BadRequest().body(err))
    }
}
