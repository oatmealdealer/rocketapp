#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate openssl;
#[macro_use] extern crate diesel;
use rocket::response::Redirect;
use diesel::PgConnection;
pub mod schema;
pub mod models;



#[database("diesel_postgres")]
struct PgDbConn(PgConnection);


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

fn main() {
    rocket::ignite()
    .attach(PgDbConn::fairing())
    .mount("/", routes![index, home])
    .launch();
}
