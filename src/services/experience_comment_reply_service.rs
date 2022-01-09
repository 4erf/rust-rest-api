use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        experience_comment_reply::{ExperienceCommentReply, ExperienceCommentReplyDTO}
    },
};
use actix_web::{http::StatusCode, web};

pub fn find_all_by_experience(country: &String, city: &String, pool: &web::Data<Pool>) -> Result<Vec<ExperienceCommentReplyDTO>, ServiceError> {
    match ExperienceCommentReply::find_all_by_experience(country, city, &pool.get().unwrap()) {
        Ok(comments) => Ok(comments.into_iter().map(|e| e.into()).collect()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn insert(new: ExperienceCommentReplyDTO, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match ExperienceCommentReply::insert(new.into(), &pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }

}