use crate::schema::*;
use crate::PgDbConn;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Debug, Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {
    pub fn new(title: String, body: String, published: bool) -> NewPost {
        NewPost {
            title,
            body,
            published
        }
    }

    pub fn create(title: String, body: String, published: bool, conn: PgDbConn) -> Result<Self, diesel::result::Error> {
        Self::new(title, body, published).save(conn)
    }

    pub fn all(conn: PgDbConn) -> Result<Vec<Self>, diesel::result::Error> {
        posts::table.select(posts::all_columns).load::<Self>(&*conn)
    }
}

impl NewPost {
    pub fn save(&self, conn: PgDbConn) -> Result<Post, diesel::result::Error> {
        diesel::insert_into(posts::table)
        .values(self)
        .get_result::<Post>(&*conn)
    }
}
