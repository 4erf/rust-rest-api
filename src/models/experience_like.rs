use crate::{
    config::db::Connection,
    schema::experience_like::{self, dsl::*},
};
use diesel::prelude::*;
use diesel::dsl::count;
use diesel::sql_query;
use diesel::sql_types::{Text, Binary, Integer, Nullable};

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
#[table_name = "experience_like"]
pub struct ExperienceLike {
    pub user: String,
    pub country: String,
    pub city: String,
}

#[derive(QueryableByName)]
pub struct TopUserRow {
    #[sql_type = "Integer"]
    pub rank: i32,
    #[sql_type = "Text"]
    pub first_name: String,
    #[sql_type = "Text"]
    pub email: String,
    #[sql_type = "Text"]
    pub last_name: String,
    #[sql_type = "Nullable<Binary>"]
    pub profile_pic: Option<Vec<u8>>,
}

#[derive(Serialize)]
pub struct TopUserRowDTO {
    pub rank: i32,
    pub first_name: String,
    pub email: String,
    pub last_name: String,
    pub profile_pic: Option<String>,
}

impl From<TopUserRow> for TopUserRowDTO {
    fn from(tu: TopUserRow) -> Self {
        Self {
            rank: tu.rank,
            email: tu.email,
            first_name: tu.first_name,
            last_name: tu.last_name,
            profile_pic: if let Some(pic) = tu.profile_pic { Some(base64::encode(pic)) } else { None },
        }
    }
}

impl ExperienceLike {
    pub fn count_all_by_experience(country_: &String, city_: &String, conn: &Connection) -> QueryResult<i64> {
        experience_like
            .select(count(user))
            .filter(country.eq(country_))
            .filter(city.eq(city_))
            .first::<i64>(conn)
    }

    pub fn get_top_users(conn: &Connection) -> QueryResult<Vec<TopUserRow>> {
        sql_query("
            select u.email, u.first_name, u.last_name, u.profile_pic, count('x') as rank
            from user u
            inner join experience e on u.email = e.author
            inner join experience_like el on e.country = el.country and e.city = el.city
            group by u.email
            order by rank desc;
        ").load(conn)
    }

    pub fn get_user_likes(email: &String, conn: &Connection) -> QueryResult<i64> {
        experience_like
            .select(count(user))
            .filter(user.eq(email))
            .first::<i64>(conn)
    }

    pub fn has_user_liked_experience(email: &String, country_: &String, city_: &String, conn: &Connection) -> QueryResult<bool> {
        let result: QueryResult<i64> = experience_like
            .select(count(user))
            .filter(user.eq(email))
            .filter(country.eq(country_))
            .filter(city.eq(city_))
            .first::<i64>(conn);

        result.map(|count| count == 1)
    }

    pub fn insert(new: ExperienceLike, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(experience_like)
            .values(&new)
            .execute(conn)
    }

    pub fn delete(pk: &(String, String, String), conn: &Connection) -> QueryResult<usize> {
        diesel::delete(experience_like.find(pk.clone())).execute(conn)
    }
}
