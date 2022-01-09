use crate::{
    config::db::Connection,
    schema::collection::{self, dsl::*},
};
use diesel::prelude::*;
use std::convert::{From, TryFrom};

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
#[table_name = "collection"]
pub struct Collection {
    pub author: String,
    pub name: String,
    pub description: String,
    pub image: Vec<u8>,
    pub season: String,
}

#[derive(Serialize, Deserialize)]
pub struct CollectionDTO {
    pub author: String,
    pub name: String,
    pub description: String,
    pub image_base64: String,
    pub season: String,
}

impl From<Collection> for CollectionDTO {
    fn from(c: Collection) -> Self {
        Self {
            author: c.author,
            name: c.name,
            description: c.description,
            image_base64: base64::encode(c.image),
            season: c.season
        }
    }
}

impl TryFrom<CollectionDTO> for Collection {
    type Error = ();

    fn try_from(c: CollectionDTO) -> Result<Self, Self::Error> {
        match base64::decode(c.image_base64) {
            Ok(decoded) => {
                Ok(Self {
                    author: c.author,
                    name: c.name,
                    description: c.description,
                    image: decoded,
                    season: c.season,
                })
            },
            Err(_) => {
                Err(())
            },
        }
    }
}

impl Collection {
    pub fn find_by_id(name_: &String, conn: &Connection) -> QueryResult<Collection> {
        collection.find(name_).get_result::<Collection>(conn)
    }

    pub fn find_all(conn: &Connection) -> QueryResult<Vec<Collection>> {
        collection.order(name).load::<Collection>(conn)
    }

    pub fn find_by_author(author_: &String, conn: &Connection) -> QueryResult<Vec<Collection>> {
        collection
            .filter(author.eq(author_))
            .order(name.asc())
            .load::<Collection>(conn)
    }

    pub fn insert(new: Collection, conn: &Connection) -> QueryResult<usize> {
        diesel::insert_into(collection)
            .values(&new)
            .execute(conn)
    }

    pub fn delete(name_: &String, conn: &Connection) -> QueryResult<usize> {
        diesel::delete(collection.find(name_)).execute(conn)
    }
}
