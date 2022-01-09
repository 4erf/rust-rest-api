use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        experience::Experience,
        experience::ExperienceDTO
    },
};
use actix_web::{http::StatusCode, web};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<ExperienceDTO>, ServiceError> {
    match Experience::find_all(&pool.get().unwrap()) {
        Ok(experiences) => Ok(experiences.into_iter().map(|e| e.into()).collect()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn search(search_params: (&String, &String, &String), pool: &web::Data<Pool>) -> Result<Vec<ExperienceDTO>, ServiceError> {
    match Experience::search(search_params, &pool.get().unwrap()) {
        Ok(experiences) => Ok(experiences.into_iter().map(|e| e.into()).collect()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_user(email: &String, pool: &web::Data<Pool>) -> Result<Vec<ExperienceDTO>, ServiceError> {
    match Experience::find_by_user(email, &pool.get().unwrap()) {
        Ok(experiences) => Ok(experiences.into_iter().map(|e| e.into()).collect()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_interests(interests: &String, pool: &web::Data<Pool>) -> Result<Vec<ExperienceDTO>, ServiceError> {
    match Experience::find_by_interests(interests, &pool.get().unwrap()) {
        Ok(experiences) => Ok(experiences.into_iter().map(|e| e.into()).collect()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_id(params: (String, String), pool: &web::Data<Pool>) -> Result<ExperienceDTO, ServiceError> {
    match Experience::find_by_id(params.clone(), &pool.get().unwrap()) {
        Ok(experience) => Ok(experience.into()),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Experience with country {} and city {} not found", params.0, params.1),
        )),
    }
}

pub fn insert(new_experience: ExperienceDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match new_experience.try_into() {
        Ok(converted) => match Experience::insert(converted, &pool.get().unwrap()) {
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

pub fn update(
    params: (String, String),
    updated_experience: ExperienceDTO,
    pool: &web::Data<Pool>,
) -> Result<(), ServiceError> {
    match Experience::find_by_id(params.clone(), &pool.get().unwrap()) {
        Ok(_) => match updated_experience.try_into() {
            Ok(converted) => match Experience::update(params.clone(), converted, &pool.get().unwrap()) {
                Ok(_) => Ok(()),
                Err(_) => Err(ServiceError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
                )),
            },
            Err(_) => Err(ServiceError::new(
                StatusCode::BAD_REQUEST,
                format!("Image has invalid format"),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Experience with country {} and city {} not found", params.0, params.1),
        )),
    }
}

pub fn delete(params: (String, String), pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Experience::find_by_id(params.clone(), &pool.get().unwrap()) {
        Ok(_) => match Experience::delete(params.clone(), &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Experience with country {} and city {} not found", params.0, params.1),
        )),
    }
}