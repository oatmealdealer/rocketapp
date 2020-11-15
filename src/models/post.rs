use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "posts"]
pub struct NewPost<'x> {
    pub title: &'x str,
    pub body: &'x str,
    pub published: bool,
}
