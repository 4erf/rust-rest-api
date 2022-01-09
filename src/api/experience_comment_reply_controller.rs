use crate::{
    config::db::Pool,
    constants,
    models::{
        experience_comment_reply::{ExperienceCommentReplyDTO},
        response::ResponseBody
    },
    services::{experience_comment_reply_service},
    utils::token_utils
};
use actix_web::{web, HttpResponse, Result};

// GET api/experience
pub async fn find_all_by_experience(web::Path((country, city)): web::Path<(String, String)>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match experience_comment_reply_service::find_all_by_experience(&country, &city, &pool) {
        Ok(count) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, count))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/experience
pub async fn insert(
    new: web::Json<ExperienceCommentReplyDTO>,
    req: web::HttpRequest,
    web::Path((email_, timestamp)): web::Path<(String, i64)>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    match token_utils::get_token_email(req) {
        Ok(email) => {
            let new = ExperienceCommentReplyDTO {
                comment_author: email_,
                comment_time: timestamp,
                reply_author: email,
                reply_time: None,
                reply_text: new.reply_text.clone(),
            };
            match experience_comment_reply_service::insert(new, &pool) {
                Ok(()) => Ok(HttpResponse::Created()
                    .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
                Err(err) => Ok(err.response()),
            }
        },
        Err(err) => Ok(HttpResponse::BadRequest().body(err))
    }
}