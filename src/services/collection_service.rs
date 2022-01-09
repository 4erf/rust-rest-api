use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        collection::Collection,
        collection::CollectionDTO
    },
};
use actix_web::{http::StatusCode, web};

pub fn find_by_id(name: &String, pool: &web::Data<Pool>) -> Result<CollectionDTO, ServiceError> {
    match Collection::find_by_id(name, &pool.get().unwrap()) {
        Ok(item) => Ok(item.into()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_author(email: &String, pool: &web::Data<Pool>) -> Result<Vec<CollectionDTO>, ServiceError> {
    match Collection::find_by_author(email, &pool.get().unwrap()) {
        Ok(items) => Ok(items.into_iter().map(|e| e.into()).collect()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<CollectionDTO>, ServiceError> {
    match Collection::find_all(&pool.get().unwrap()) {
        Ok(items) => Ok(items.into_iter().map(|e| e.into()).collect()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn insert(new: CollectionDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match new.try_into() {
        Ok(converted) => match Collection::insert(converted, &pool.get().unwrap()) {
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

pub fn delete(name: &String, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Collection::find_by_id(name, &pool.get().unwrap()) {
        Ok(_) => match Collection::delete(name, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Experience with name {} not found", name),
        )),
    }
}