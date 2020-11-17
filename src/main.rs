#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
extern crate serde;
use serde::Serialize;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use diesel::PgConnection;
use diesel::connection::Connection;

pub mod schema;
pub mod models;
use models::post::Post;
#[database("diesel_postgres")]
pub struct PgDbConn(PgConnection);

// TODO
#[derive(Serialize)]
struct Context {
    posts: Option<Vec<Post>>
}

#[get("/")]
fn index(conn: PgDbConn) -> Template {
    let context = Context {
        posts: match Post::all(conn) {
            Ok(posts) => Some(posts),
            Err(_) => None
        }
    };
    Template::render("index", context)
}


pub const DEFAULT_ERROR_MSG: &str = "An unknown error occurred.";

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

    rocket
    .attach(PgDbConn::fairing())
    .attach(Template::fairing())
    .mount("/", routes![index])
    .mount("/public", StaticFiles::from("/static"))
    .mount("/posts", routes![
        models::post::routes::create_one,
        models::post::routes::create_many,
        models::post::routes::get_one,
        models::post::routes::get_all,
        models::post::routes::update_one,
        models::post::routes::delete,
    ])
    .launch();
}
