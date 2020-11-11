#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;

#[macro_use] extern crate rocket;

#[get("/<greeting>/<person>")]
fn index(person: &RawStr, greeting: &RawStr) -> String {
    format!("{} {}!", greeting.as_str(), person.as_str())
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
