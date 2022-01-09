use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        experience_image::ExperienceImage,
        experience_image::ExperienceImageDTO
    },
};
use actix_web::{http::StatusCode, web};

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<ExperienceImageDTO, ServiceError> {
    match ExperienceImage::find_by_id(id, &pool.get().unwrap()) {
        Ok(experience) => Ok(experience.into()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_all_by_experience(country: &String, city: &String, pool: &web::Data<Pool>) -> Result<Vec<ExperienceImageDTO>, ServiceError> {
    match ExperienceImage::find_all_by_experience(country, city, &pool.get().unwrap()) {
        Ok(experiences) => Ok(experiences.into_iter().map(|e| e.into()).collect()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn insert(new_experience: ExperienceImageDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match new_experience.try_into() {
        Ok(converted) => match ExperienceImage::insert(converted, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            format!("Image has invalid format"),
        )),
    }

}

pub fn delete(id: i32, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match ExperienceImage::find_by_id(id, &pool.get().unwrap()) {
        Ok(_) => match ExperienceImage::delete(id, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Experience with id {} not found", id),
        )),
    }
}