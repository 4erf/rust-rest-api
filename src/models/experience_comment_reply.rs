use crate::{
    config::db::Connection,
    schema::experience_comment_reply::{self, dsl::*},
    schema::experience_comment,
};
use diesel::prelude::*;

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
#[table_name = "experience_comment_reply"]
pub struct ExperienceCommentReply {
    pub comment_author: String,
    pub comment_time: i64,
    pub reply_author: String,
    pub reply_time: i64,
    pub reply_text: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExperienceCommentReplyDTO {
    pub comment_author: String,
    pub comment_time: i64,
    pub reply_author: String,
    pub reply_time: Option<i64>,
    pub reply_text: String,
}

impl From<ExperienceCommentReply> for ExperienceCommentReplyDTO {
    fn from(ec: ExperienceCommentReply) -> Self {
        Self {
            comment_author: ec.comment_author,
            comment_time: ec.comment_time,
            reply_author: ec.reply_author,
            reply_time: Some(ec.reply_time),
            reply_text: ec.reply_text,
        }
    }
}

impl From<ExperienceCommentReplyDTO> for ExperienceCommentReply {
    fn from(ec: ExperienceCommentReplyDTO) -> Self {
        Self {
            comment_author: ec.comment_author,
            comment_time: ec.comment_time,
            reply_author: ec.reply_author,
            reply_time: ec.reply_time.unwrap_or(chrono::offset::Utc::now().timestamp()),
            reply_text: ec.reply_text,
        }
    }
}

impl ExperienceCommentReply {
    pub fn find_all_by_experience(country_: &String, city_: &String, conn: &Connection) -> QueryResult<Vec<ExperienceCommentReply>> {
        experience_comment_reply
            .select(experience_comment_reply::all_columns)
            .inner_join(experience_comment::table.on(
                comment_author.eq(experience_comment::dsl::author)
                    .and(comment_time.eq(experience_comment::dsl::timestamp))
            ))
            .filter(experience_comment::dsl::country.eq(country_))
            .filter(experience_comment::dsl::city.eq(city_))
            .load::<ExperienceCommentReply>(conn)
    }

    pub fn insert(new: ExperienceCommentReply, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(experience_comment_reply)
            .values(&new)
            .execute(conn)
    }
}
