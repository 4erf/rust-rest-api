use crate::{
    config::db::Connection,
    schema::experience_image::{self, dsl::*},
};
use diesel::prelude::*;
use std::convert::{From, TryFrom};

#[derive(Queryable, Serialize, Deserialize)]
pub struct ExperienceImage {
    pub id: i32,
    pub country: String,
    pub city: String,
    pub image: Vec<u8>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "experience_image"]
pub struct ExperienceImageInsert {
    pub country: String,
    pub city: String,
    pub image: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct ExperienceImageDTO {
    pub id: Option<i32>,
    pub country: String,
    pub city: String,
    pub image_base64: String,
}

impl From<ExperienceImage> for ExperienceImageDTO {
    fn from(ei: ExperienceImage) -> Self {
        Self {
            id: Some(ei.id),
            city: ei.city,
            country: ei.country,
            image_base64: base64::encode(ei.image),
        }
    }
}

impl TryFrom<ExperienceImageDTO> for ExperienceImageInsert {
    type Error = ();

    fn try_from(ei: ExperienceImageDTO) -> Result<Self, Self::Error> {
        match base64::decode(ei.image_base64) {
            Ok(decoded) => {
                Ok(Self {
                    city: ei.city,
                    country: ei.country,
                    image: decoded,
                })
            },
            Err(_) => {
                Err(())
            },
        }

    }
}

impl ExperienceImage {
    pub fn find_by_id(id_: i32, conn: &Connection) -> QueryResult<ExperienceImage> {
        experience_image.find(id_).get_result::<ExperienceImage>(conn)
    }

    pub fn find_all_by_experience(country_: &String, city_: &String, conn: &Connection) -> QueryResult<Vec<ExperienceImage>> {
        experience_image
            .filter(country.eq(country_))
            .filter(city.eq(city_))
            .order(id.asc())
            .load::<ExperienceImage>(conn)
    }

    pub fn insert(new_image: ExperienceImageInsert, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(experience_image)
            .values(&new_image)
            .execute(conn)
    }

    pub fn delete(id_: i32, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(experience_image.find(id_)).execute(conn)
    }
}
