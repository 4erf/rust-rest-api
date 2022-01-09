use crate::{
    config::db::Connection,
    schema::experience_comment::{self, dsl::*},
};
use diesel::prelude::*;

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
#[table_name = "experience_comment"]
pub struct ExperienceComment {
    pub author: String,
    pub country: String,
    pub city: String,
    pub text: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ExperienceCommentDTO {
    pub author: String,
    pub country: String,
    pub city: String,
    pub text: String,
    pub timestamp: Option<i64>,
}

impl From<ExperienceComment> for ExperienceCommentDTO {
    fn from(ec: ExperienceComment) -> Self {
        Self {
            author: ec.author,
            city: ec.city,
            country: ec.country,
            text: ec.text,
            timestamp: Some(ec.timestamp),
        }
    }
}

impl From<ExperienceCommentDTO> for ExperienceComment {
    fn from(ec: ExperienceCommentDTO) -> Self {
        Self {
            author: ec.author,
            city: ec.city,
            country: ec.country,
            text: ec.text,
            timestamp: ec.timestamp.unwrap_or(chrono::offset::Utc::now().timestamp())
        }
    }
}

impl ExperienceComment {
    pub fn find_all_by_experience(country_: &String, city_: &String, conn: &Connection) -> QueryResult<Vec<ExperienceComment>> {
        experience_comment
            .filter(country.eq(country_))
            .filter(city.eq(city_))
            .load::<ExperienceComment>(conn)
    }

    pub fn insert(new: ExperienceComment, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(experience_comment)
            .values(&new)
            .execute(conn)
    }
}
