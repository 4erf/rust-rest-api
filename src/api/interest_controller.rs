use crate::{
    config::db::Pool,
    constants,
    models::{response::ResponseBody},
    services::{interest_service},
};
use actix_web::{web, HttpResponse, Result};

// GET api/experience
pub async fn find_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
    match interest_service::find_all(&pool) {
        Ok(items) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, items))),
        Err(err) => Ok(err.response()),
    }
}