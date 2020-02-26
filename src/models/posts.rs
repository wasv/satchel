use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::connection::Conn;
use crate::schema::posts;

#[derive(Insertable, AsChangeset, Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

pub fn create_post(post: Post, connection: &Conn) -> Post {
    diesel::insert_into(posts::table)
        .values(&post)
        .execute(connection)
        .expect("Error creating new post");

    posts::table
        .order(posts::id.desc())
        .first(connection)
        .unwrap()
}

pub fn read_post(connection: &Conn) -> Vec<Post> {
    posts::table
        .order(posts::id.asc())
        .load::<Post>(connection)
        .unwrap()
}

pub fn update_post(id: i32, post: Post, connection: &Conn) -> bool {
    diesel::update(posts::table.find(id))
        .set(&post)
        .execute(connection)
        .is_ok()
}

pub fn delete_post(id: i32, connection: &Conn) -> bool {
    diesel::delete(posts::table.find(id))
        .execute(connection)
        .is_ok()
}

