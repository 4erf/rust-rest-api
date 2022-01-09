use crate::{
    config::db::Connection,
    schema::interest::{self, dsl::*},
};
use diesel::prelude::*;

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
#[table_name = "interest"]
pub struct Interest {
    pub name: String,
}

impl Interest {
    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Interest>> {
        interest
            .order(name.asc())
            .load::<Interest>(conn)
    }
}
