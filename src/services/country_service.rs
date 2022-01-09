use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        country::Country,
    },
};
use actix_web::{http::StatusCode, web};

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<Country>, ServiceError> {
    match Country::find_all(&pool.get().unwrap()) {
        Ok(items) => Ok(items),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}