use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        experience_interest::ExperienceInterest,
    },
};
use actix_web::{http::StatusCode, web};

pub fn find_by_experience(country: &String, city: &String, pool: &web::Data<Pool>) -> Result<Vec<ExperienceInterest>, ServiceError> {
    match ExperienceInterest::find_by_experience(country, city, &pool.get().unwrap()) {
        Ok(items) => Ok(items.into_iter().map(|e| e.into()).collect()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn insert(new: ExperienceInterest, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match ExperienceInterest::insert(new, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}