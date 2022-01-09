use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        experience_like::{ExperienceLike, TopUserRow}
    },
};
use actix_web::{http::StatusCode, web};

pub fn count_all_by_experience(country: &String, city: &String, pool: &web::Data<Pool>) -> Result<i64, ServiceError> {
    match ExperienceLike::count_all_by_experience(country, city, &pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn get_top_users(pool: &web::Data<Pool>) -> Result<Vec<TopUserRow>, ServiceError> {
    match ExperienceLike::get_top_users(&pool.get().unwrap()) {
        Ok(top) => Ok(top),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn get_user_likes(email: &String, pool: &web::Data<Pool>) -> Result<i64, ServiceError> {
    match ExperienceLike::get_user_likes(email, &pool.get().unwrap()) {
        Ok(count) => Ok(count),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn has_user_liked_experience(email: &String, country: &String, city: &String, pool: &web::Data<Pool>) -> Result<bool, ServiceError> {
    match ExperienceLike::has_user_liked_experience(email, country, city, &pool.get().unwrap()) {
        Ok(liked) => Ok(liked),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn insert(new: ExperienceLike, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match ExperienceLike::insert(new, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }

}

pub fn delete(pk: &(String, String, String), pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match ExperienceLike::delete(pk, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
        )),
    }
}