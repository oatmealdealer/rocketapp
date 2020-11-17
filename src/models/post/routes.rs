use rocket_contrib::json::Json;
use rocket::response::status::NotFound;

use crate::{PgDbConn, DEFAULT_ERROR_MSG};
use crate::models::post::{Post, NewPost, UpdatePost};

#[post("/", data="<post>")]
pub fn create_one(post: Json<NewPost>, db: PgDbConn) -> Result<Json<Post>, &'static str> {
    match post.into_inner().save(db) {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(DEFAULT_ERROR_MSG)
    }
}

// #[post("/posts", data="<new_posts>", rank=2)]
// IDEA: Add a single new_post parameter *and* a new_posts parameter
// and make them both optional
#[post("/bulk_create", data="<new_posts>")]
pub fn create_many(new_posts: Json<Vec<NewPost>>, conn: PgDbConn) -> Result<Json<Vec<Post>>, &'static str> {
    match Post::save(new_posts.into_inner(), conn) {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(DEFAULT_ERROR_MSG)
    }
}

#[get("/<id>")]
pub fn get_one(conn: PgDbConn, id: i32) -> Result<Json<Post>, NotFound<&'static str>> {
    match Post::retrieve(id, conn) {
        Some(post) => Ok(Json(post)),
        None => Err(NotFound("Not found."))
    }
}

#[get("/")]
pub fn get_all(conn: PgDbConn) -> Result<Json<Vec<Post>>, &'static str> {
    match Post::all(conn) {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(DEFAULT_ERROR_MSG)
    }
}

#[patch("/", data="<updated_post>")]
pub fn update_one(conn: PgDbConn, updated_post: Json<UpdatePost>) -> Result<Json<Post>, NotFound<&'static str>> {
    match Post::update(updated_post.into_inner(), conn) {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(NotFound("Not found."))
    }
}

#[delete("/<id>")]
pub fn delete(conn: PgDbConn, id: i32) -> Result<(), NotFound<&'static str>> {
    // This doesn't return an error if the id doesn't exist
    match Post::delete(id, conn) {
        Ok(_) => Ok(()),
        Err(_) => Err(NotFound("Not found."))
    }
}
