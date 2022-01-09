use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        collection_experience::CollectionExperience,
    },
};
use actix_web::{http::StatusCode, web};

pub fn find_by_collection(name: &String, pool: &web::Data<Pool>) -> Result<Vec<CollectionExperience>, ServiceError> {
    match CollectionExperience::find_by_collection(name, &pool.get().unwrap()) {
        Ok(items) => Ok(items.into_iter().map(|e| e.into()).collect()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn insert(new: CollectionExperience, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match CollectionExperience::insert(new, &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn delete(pk: &(String, String, String), pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match CollectionExperience::find_by_id(pk, &pool.get().unwrap()) {
        Ok(_) => match CollectionExperience::delete(pk, &pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Experience with name, country and city {}, {}, {} not found", pk.0, pk.1, pk.2),
        )),
    }
}