use crate::schema::*;
use crate::PgDbConn;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

#[derive(Debug, Serialize, Queryable, Identifiable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
// Maybe I need to manually implement deserialization for this
#[derive(Deserialize, Serialize)]
pub enum OneOrMore<T> {
    One(T),
    More(Vec<T>)
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Deserialize, AsChangeset, Identifiable)]
#[table_name = "posts"]
pub struct UpdatePost {
    pub id: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>
}

pub mod routes;

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

    pub fn retrieve(id: i32, conn: PgDbConn) -> Option<Self> {
        match posts::table
        .select(posts::all_columns)
        .filter(posts::id.eq(id))
        .first::<Self>(&*conn) {
            Ok(post) => Some(post),
            Err(_) => None
        }
    }

    pub fn update(update_post: UpdatePost, conn: PgDbConn) -> Result<Self, diesel::result::Error> {
        diesel::update(&update_post)
        .set(&update_post)
        .get_result(&*conn)
    }

    pub fn delete(id: i32, conn: PgDbConn) -> Result<(), diesel::result::Error> {
        match diesel::delete(posts::table)
        .filter(posts::id.eq(id))
        .execute(&*conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    pub fn all(conn: PgDbConn) -> Result<Vec<Self>, diesel::result::Error> {
        posts::table.select(posts::all_columns).load::<Self>(&*conn)
    }

    pub fn save(new_posts: Vec<NewPost>, conn: PgDbConn) -> Result<Vec<Self>, diesel::result::Error> {
        // use crate::schema::posts::dsl::*;
        diesel::insert_into(posts::table)
        .values(&new_posts)
        // .returning(posts::dsl::id)
        .get_results(&*conn)
    }
}

impl NewPost {
    pub fn save(&self, conn: PgDbConn) -> Result<Post, diesel::result::Error> {
        diesel::insert_into(posts::table)
        .values(self)
        .get_result::<Post>(&*conn)
    }
}
