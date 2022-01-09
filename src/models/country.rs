use crate::{
    config::db::Connection,
    schema::country::{self, dsl::*},
};
use diesel::prelude::*;

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
#[table_name = "country"]
pub struct Country {
    pub name: String,
}

impl Country {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Country>> {
        country
            .order(name.asc())
            .load::<Country>(conn)
    }
}
