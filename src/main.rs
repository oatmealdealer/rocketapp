#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate serde;
use rocket_contrib::json::Json;
use rocket::response::{Redirect, status};

#[macro_use] extern crate diesel_migrations;
use diesel::PgConnection;
use diesel::connection::Connection;
use diesel::prelude::*;

pub mod schema;
pub mod models {
    pub mod post;
}
use models::post::{Post, NewPost};

#[database("diesel_postgres")]
pub struct PgDbConn(PgConnection);


pub const DEFAULT_ERROR_MSG: &str = "An unknown error occurred.";


#[get("/<greeting>/<person>")]
fn index(person: String, greeting: String) -> String {
    format!("{} {}!", greeting.as_str(), person.as_str())
}

#[get("/")]
fn home() -> Redirect {
    let greeting: &str = "Hello";
    let person: &str = "stranger";
    Redirect::to(uri!(index: person = String::from(person), greeting = String::from(greeting)))
}



#[post("/posts", data="<post>")]
fn new_post(post: Json<NewPost>, db: PgDbConn) -> Result<Json<Post>, &'static str> {
    match post.into_inner().save(db) {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(DEFAULT_ERROR_MSG)
    }
}

// #[post("/posts", data="<new_posts>", rank=2)]
#[post("/posts/bulk_create", data="<new_posts>")]
fn new_posts(new_posts: Json<Vec<NewPost>>, conn: PgDbConn) -> Result<Json<Vec<Post>>, &'static str> {
    match Post::save(new_posts.into_inner(), conn) {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(DEFAULT_ERROR_MSG)
    }
}

#[get("/posts")]
fn list_posts(db: PgDbConn) -> Result<Json<Vec<Post>>, &'static str> {
    match Post::all(db) {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(DEFAULT_ERROR_MSG)
    }
}

#[get("/posts/<id>")]
fn get_post(db: PgDbConn, id: i32) -> Result<Json<Post>, status::NotFound<&'static str>> {
    match Post::find(id, db) {
        Some(post) => Ok(Json(post)),
        None => Err(status::NotFound("Not found."))
    }
}

#[delete("/posts/<id>")]
fn delete_post(conn: PgDbConn, id: i32) -> Result<(), status::NotFound<&'static str>> {
    // This doesn't return an error if the id doesn't exist
    match Post::delete(id, conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(status::NotFound("Not found."))
    }
}


diesel_migrations::embed_migrations!("./migrations");


fn main() {
    let rocket = rocket::ignite();
    let db_conf = rocket_contrib::databases::database_config("diesel_postgres", rocket.config())
    .expect("Unable to read DB config");

    let db_url = db_conf.url;
    let temp_db_conn = PgConnection::establish(db_url)
    .expect("Failed to establish initial database connection");

    embedded_migrations::run(&temp_db_conn)
    .expect("Failed to run migrations");

    rocket.attach(PgDbConn::fairing())
    .mount("/", routes![new_post, new_posts, list_posts, get_post, delete_post])
    .launch();
}
