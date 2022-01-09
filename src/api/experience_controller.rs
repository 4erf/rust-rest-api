use crate::{
    config::db::Pool,
    constants,
    models::{experience::ExperienceDTO, response::ResponseBody},
    services::experience_service,
    utils::token_utils
};
use actix_web::{web, HttpResponse, Result};

#[derive(Deserialize)]
pub struct FindQuery {
    email: Option<String>,
    interests: Option<String>,
    author: Option<String>,
    interest: Option<String>,
    place: Option<String>,
}

// GET api/experience
pub async fn find_all(query: web::Query<FindQuery>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    if let Some(_) = &query.author {
        return match experience_service::search((&query.author.as_ref().unwrap(), &query.interest.as_ref().unwrap(), &query.place.as_ref().unwrap()), &pool) {
            Ok(experiences) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, experiences))),
            Err(err) => Ok(err.response()),
        }
    }
    if let Some(email) = &query.email {
        return match experience_service::find_by_user(email, &pool) {
            Ok(experiences) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, experiences))),
            Err(err) => Ok(err.response()),
        }
    }
    if let Some(interests) = &query.interests {
        return match experience_service::find_by_interests(interests, &pool) {
            Ok(experiences) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, experiences))),
            Err(err) => Ok(err.response()),
        }
    }
    match experience_service::find_all(&pool) {
        Ok(experiences) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, experiences))),
        Err(err) => Ok(err.response()),
    }
}

// GET api/experience/{country}/{city}
pub async fn find_by_id(params: web::Path<(String, String)>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match experience_service::find_by_id(params.into_inner(), &pool) {
        Ok(experience) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, experience))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/experience
pub async fn insert(
    req: web::HttpRequest,
    mut new_experience: web::Json<ExperienceDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => {
            new_experience.author = email;
            match experience_service::insert(new_experience.0, &pool) {
                Ok(()) => Ok(HttpResponse::Created()
                    .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
                Err(err) => Ok(err.response()),
            }
        },
        Err(err) => Ok(HttpResponse::BadRequest().body(err))
    }

}

// PUT api/experience/{country}/{city}
pub async fn update(
    req: web::HttpRequest,
    params: web::Path<(String, String)>,
    mut updated_experience: web::Json<ExperienceDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => match experience_service::find_by_id(params.clone(), &pool) {
            Ok(experience) => {
                if experience.author == email {
                    updated_experience.author = email;
                    match experience_service::update(params.clone(),updated_experience.0, &pool) {
                        Ok(()) => {
                            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
                        }
                        Err(err) => Ok(err.response()),
                    }
                } else {
                    Ok(HttpResponse::BadRequest().json("This experience does not belong to this user"))
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
    params: web::Path<(String, String)>,
    pool: web::Data<Pool>
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => match experience_service::find_by_id(params.clone(), &pool) {
            Ok(experience) => {
                if experience.author == email {
                    match experience_service::delete(params.clone(), &pool) {
                        Ok(()) => {
                            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
                        }
                        Err(err) => Ok(err.response()),
                    }
                } else {
                    Ok(HttpResponse::BadRequest().json("This experience does not belong to this user"))
                }
            },
            Err(err) => Ok(err.response()),
        },
        Err(err) => Ok(HttpResponse::BadRequest().body(err))
    }
}
