use crate::{
    config::db::Pool,
    constants,
    models::{experience_image::ExperienceImageDTO, response::ResponseBody},
    services::{experience_image_service, experience_service},
    utils::token_utils
};
use actix_web::{web, HttpResponse, Result};

// GET api/experience
pub async fn find_all_by_experience(web::Path((country, city)): web::Path<(String, String)>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match experience_image_service::find_all_by_experience(&country, &city, &pool) {
        Ok(experiences) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, experiences))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/experience
pub async fn insert(
    req: web::HttpRequest,
    new_image: web::Json<ExperienceImageDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => match experience_service::find_by_id((new_image.country.clone(), new_image.city.clone()), &pool) {
            Ok(experience) => {
                if experience.author == email {
                    match experience_image_service::insert(new_image.0, &pool) {
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

// DELETE api/experience/{country}/{city}
pub async fn delete(
    req: web::HttpRequest,
    id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => match experience_image_service::find_by_id(id.clone(), &pool) {
            Ok(image) => match experience_service::find_by_id((image.country.clone(), image.city.clone()), &pool) {
                Ok(experience) => {
                    if experience.author == email {
                        match experience_image_service::delete(id.clone(), &pool) {
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
            Err(err) => Ok(err.response()),
        },
        Err(err) => Ok(HttpResponse::BadRequest().body(err))
    }
}
