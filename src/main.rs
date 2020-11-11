#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
use diesel::PgConnection;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[database("diesel_postgres")]
struct PgDbConn(PgConnection);


#[get("/<greeting>/<person>")]
fn index(person: &RawStr, greeting: &RawStr) -> String {
    format!("{} {}!", greeting.as_str(), person.as_str())
}

fn main() {
    rocket::ignite().attach(PgDbConn::fairing()).mount("/", routes![index]).launch();
}
