use crate::{
    config::db::Connection,
    schema::experience_interest::{self, dsl::*},
};
use diesel::prelude::*;

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
#[table_name = "experience_interest"]
pub struct ExperienceInterest {
    pub interest: String,
    pub country: String,
    pub city: String,
}

impl ExperienceInterest {
    pub fn find_by_experience(country_: &String, city_: &String, conn: &Connection) -> QueryResult<Vec<ExperienceInterest>> {
        experience_interest
            .filter(country.eq(country_))
            .filter(city.eq(city_))
            .order(country.asc())
            .load::<ExperienceInterest>(conn)
    }

    pub fn insert(new: ExperienceInterest, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(experience_interest)
            .values(&new)
            .execute(conn)
    }
}
