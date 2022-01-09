use crate::{
    config::db::Connection,
    schema::collection_experience::{self, dsl::*},
};
use diesel::prelude::*;

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
#[table_name = "collection_experience"]
pub struct CollectionExperience {
    pub name: String,
    pub country: String,
    pub city: String,
}

impl CollectionExperience {
    pub fn find_by_id(pk: &(String, String, String), conn: &Connection) -> QueryResult<CollectionExperience> {
        collection_experience.find(pk.clone()).get_result::<CollectionExperience>(conn)
    }

    pub fn find_by_collection(name_: &String, conn: &Connection) -> QueryResult<Vec<CollectionExperience>> {
        collection_experience
            .filter(name.eq(name_))
            .order(name.asc())
            .load::<CollectionExperience>(conn)
    }

    pub fn insert(new: CollectionExperience, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(collection_experience)
            .values(&new)
            .execute(conn)
    }

    pub fn delete(pk: &(String, String, String), conn: &Connection) -> QueryResult<usize> {
        diesel::delete(collection_experience.find(pk.clone())).execute(conn)
    }
}
