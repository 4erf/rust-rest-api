use crate::{
    config::db::Connection,
    schema::experience_video::{self, dsl::*},
};
use diesel::prelude::*;
use std::convert::{From};

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
#[table_name = "experience_video"]
pub struct ExperienceVideo {
    pub id: i32,
    pub country: String,
    pub city: String,
    pub video_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExperienceVideoDTO {
    pub id: Option<i32>,
    pub country: String,
    pub city: String,
    pub video_url: String,
}

impl From<ExperienceVideo> for ExperienceVideoDTO {
    fn from(ev: ExperienceVideo) -> Self {
        Self {
            id: Some(ev.id),
            city: ev.city,
            country: ev.country,
            video_url: ev.video_url,
        }
    }
}

impl From<ExperienceVideoDTO> for ExperienceVideo {
    fn from(ev: ExperienceVideoDTO) ->Self {
        Self {
            id: 0,
            city: ev.city,
            country: ev.country,
            video_url: ev.video_url,
        }
    }
}

impl ExperienceVideo {
    pub fn find_by_id(id_: i32, conn: &Connection) -> QueryResult<ExperienceVideo> {
        experience_video.find(id_).get_result::<ExperienceVideo>(conn)
    }

    pub fn find_all_by_experience(country_: &String, city_: &String, conn: &Connection) -> QueryResult<Vec<ExperienceVideo>> {
        experience_video
            .filter(country.eq(country_))
            .filter(city.eq(city_))
            .order(id.asc())
            .load::<ExperienceVideo>(conn)
    }

    pub fn insert(new: ExperienceVideo, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(experience_video)
            .values(&new)
            .execute(conn)
    }

    pub fn delete(id_: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(experience_video.find(id_)).execute(conn)
    }
}
