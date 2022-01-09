use crate::{
    config::db::Pool,
    constants,
    models::{experience_interest::ExperienceInterest, response::ResponseBody},
    services::{experience_interest_service, experience_service},
    utils::token_utils
};
use actix_web::{web, HttpResponse, Result};

// GET api/experience
pub async fn find_by_experience(web::Path((country, city)): web::Path<(String, String)>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match experience_interest_service::find_by_experience(&country.clone(), &city.clone(), &pool) {
        Ok(items) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, items))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/experience
pub async fn insert(
    req: web::HttpRequest,
    new: web::Json<ExperienceInterest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => match experience_service::find_by_id((new.country.clone(), new.city.clone()), &pool) {
            Ok(experience) => {
                if experience.author == email {
                    match experience_interest_service::insert(new.0, &pool) {
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
        Err(err) => Ok(HttpResponse::BadRequest().body(err)),
    }
}
