#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
extern crate serde;
use rocket_contrib::json::Json;
use rocket::response::Redirect;

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
fn new_post(post: Json<NewPost>, db: PgDbConn) -> Result<Json<Post>, String> {
    // use crate::schema::posts::dsl::*;

    let insert = diesel::insert_into(schema::posts::table)
    .values(post.into_inner())
    .get_result::<Post>(&*db);

    match insert {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(String::from("Failed."))
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
    .mount("/", routes![index, home, new_post])
    .launch();
}
