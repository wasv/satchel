#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;

use rocket_contrib::json::{Json, JsonValue};

use satchel::connection;
use satchel::models::Post;

#[post("/", data = "<post>")]
fn create(post: Json<Post>, connection: connection::DbConn) -> Json<Post> {
    let insert = Post {
        ..post.into_inner()
    };
    Json(Post::create(insert, &connection))
}

#[get("/")]
fn read(connection: connection::DbConn) -> Json<JsonValue> {
    Json(json!(Post::read(&connection)))
}

#[put("/<id>", data = "<post>")]
fn update(id: i32, post: Json<Post>, connection: connection::DbConn) -> Json<JsonValue> {
    let update = Post {
        id,
        ..post.into_inner()
    };
    Json(json!({ "success": Post::update(id, update, &connection) }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: connection::DbConn) -> Json<JsonValue> {
    Json(json!({ "success": Post::delete(id, &connection) }))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![create,read,update,delete])
        .manage(connection::init_pool())
        .launch();
}
