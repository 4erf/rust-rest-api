use crate::{
    config::db::Connection,
    schema::experience::{self, dsl::*},
    schema::user,
    schema::experience_interest,
};
use diesel::prelude::*;
use std::convert::{From, TryFrom};

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
#[table_name = "experience"]
pub struct Experience {
    pub author: String,
    pub name: String,
    pub description: String,
    pub country: String,
    pub city: String,
    pub main_image: Vec<u8>,
    pub season: String,
    pub what_to_know: Option<String>,
    pub visa: Option<i32>,
    pub pp_validity: Option<String>,
    pub pp_pages: Option<i32>,
    pub vaccination: Option<String>,
    pub currency_entry: Option<String>,
    pub currency_exit: Option<String>,
    pub budget: Option<i32>,
    pub transport: Option<String>,
    pub additional_info: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ExperienceDTO {
    pub author: String,
    pub name: String,
    pub description: String,
    pub country: String,
    pub city: String,
    pub main_image_base64: String,
    pub season: String,
    pub what_to_know: Option<String>,
    pub visa: Option<i32>,
    pub pp_validity: Option<String>,
    pub pp_pages: Option<i32>,
    pub vaccination: Option<String>,
    pub currency_entry: Option<String>,
    pub currency_exit: Option<String>,
    pub budget: Option<i32>,
    pub transport: Option<String>,
    pub additional_info: Option<String>,
}

impl From<Experience> for ExperienceDTO {
    fn from(e: Experience) -> Self {
        Self {
            author: e.author,
            name: e.name,
            description: e.description,
            country: e.country,
            city: e.city,
            main_image_base64: base64::encode(e.main_image),
            season: e.season,
            what_to_know: e.what_to_know,
            visa: e.visa,
            pp_validity: e.pp_validity,
            pp_pages: e.pp_pages,
            vaccination: e.vaccination,
            currency_entry: e.currency_entry,
            currency_exit: e.currency_exit,
            budget: e.budget,
            transport: e.transport,
            additional_info: e.additional_info,
        }
    }
}

impl TryFrom<ExperienceDTO> for Experience {
    type Error = ();

    fn try_from(e: ExperienceDTO) -> Result<Self, Self::Error> {
        match base64::decode(e.main_image_base64) {
            Ok(decoded) => {
                Ok(Self {
                    author: e.author,
                    name: e.name,
                    description: e.description,
                    country: e.country,
                    city: e.city,
                    main_image: decoded,
                    season: e.season,
                    what_to_know: e.what_to_know,
                    visa: e.visa,
                    pp_validity: e.pp_validity,
                    pp_pages: e.pp_pages,
                    vaccination: e.vaccination,
                    currency_entry: e.currency_entry,
                    currency_exit: e.currency_exit,
                    budget: e.budget,
                    transport: e.transport,
                    additional_info: e.additional_info,
                })
            },
            Err(_) => {
                Err(())
            },
        }

    }
}

impl Experience {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Experience>> {
        experience.order(country.asc()).load::<Experience>(conn)
    }

    pub fn search((author_, interest_, place_): (&String, &String, &String), conn: &Connection) -> QueryResult<Vec<Experience>> {
        experience
            .select(experience::all_columns)
            .distinct()
            .inner_join(user::table)
            .inner_join(experience_interest::table.on(
                country.eq(experience_interest::dsl::country)
                    .and(city.eq(experience_interest::dsl::city)),
            ))
            .filter(
                name.like(format!("%{}%", place_))
                    .or(country.like(format!("%{}%", place_)))
                    .or(city.like(format!("%{}%", place_)))
            )
            .filter(
                user::dsl::first_name.like(format!("%{}%", author_))
                    .or(user::dsl::last_name.like(format!("%{}%", author_)))
            )
            .filter(experience_interest::dsl::interest.like(format!("%{}%", interest_)))
            .order(country.asc())
            .load::<Experience>(conn)
    }

    pub fn find_by_user(email: &String, conn: &Connection) -> QueryResult<Vec<Experience>> {
        experience
            .select(experience::all_columns)
            .inner_join(user::table)
            .filter(author.eq(email))
            .order(country.asc())
            .load::<Experience>(conn)
    }

    pub fn find_by_interests(interests: &String, conn: &Connection) -> QueryResult<Vec<Experience>> {

        let mut base_query = experience
            .select(experience::all_columns)
            .inner_join(experience_interest::table.on(
                country.eq(experience_interest::dsl::country)
                    .and(city.eq(experience_interest::dsl::city)),
            ))
            .into_boxed();

        for interest in interests.split(',' ) {
            base_query = base_query.or_filter(experience_interest::dsl::interest.eq(interest))
        }

        base_query
            .order(country.asc())
            .load::<Experience>(conn)
    }

    pub fn find_by_id(id: (String, String), conn: &Connection) -> QueryResult<Experience> {
        experience.find(id).get_result::<Experience>(conn)
    }

    pub fn insert(new_experience: Experience, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(experience)
            .values(&new_experience)
            .execute(conn)
    }

    pub fn update(id: (String, String), updated_experience: Experience, conn: &Connection) -> QueryResult<usize> {
        diesel::update(experience.find(id))
            .set(&updated_experience)
            .execute(conn)
    }

    pub fn delete(id: (String, String), conn: &Connection) -> QueryResult<usize> {
        diesel::delete(experience.find(id)).execute(conn)
    }
}
