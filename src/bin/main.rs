#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

#[macro_use]
extern crate rocket_contrib;
extern crate serde_json;

use rocket_contrib::json::{Json, JsonValue};

use satchel::connection;
use satchel::models::posts::*;

#[post("/", data = "<post>")]
fn create(post: Json<NewPost>, connection: connection::DbConn) -> Json<Post> {
    let insert = NewPost {
        ..post.into_inner()
    };
    Json(create_post(insert, &connection))
}

#[get("/")]
fn list_all(connection: connection::DbConn) -> Json<JsonValue> {
    Json(json!(list_posts(&connection)))
}

#[get("/<id>")]
fn read(id: i32, connection: connection::DbConn) -> Json<JsonValue> {
    Json(json!(read_post(id, &connection)))
}

#[put("/", data = "<post>")]
fn update(post: Json<Post>, connection: connection::DbConn) -> Json<JsonValue> {
    let update = Post {
        ..post.into_inner()
    };
    Json(json!({ "success": update_post(update, &connection) }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: connection::DbConn) -> Json<JsonValue> {
    Json(json!({ "success": delete_post(id, &connection) }))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![create, list_all, read, update, delete])
        .manage(connection::init_pool())
        .launch();
}
