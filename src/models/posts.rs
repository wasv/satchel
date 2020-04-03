use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::connection::Conn;
use crate::schema::posts;

#[derive(AsChangeset, Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub published: Option<bool>,
}

pub fn create_post(post: NewPost, connection: &Conn) -> Post {
    diesel::insert_into(posts::table)
        .values(&post)
        .execute(connection)
        .expect("Error creating new post");

    posts::table
        .order(posts::id.desc())
        .first(connection)
        .unwrap()
}

pub fn read_post(id: i32, connection: &Conn) -> Post {
    posts::table
        .find(id)
        .first(connection)
        .expect("Could not find post.")
}

pub fn list_posts(connection: &Conn) -> Vec<Post> {
    posts::table
        .order(posts::id.asc())
        .load::<Post>(connection)
        .expect("No posts found.")
}

pub fn update_post(post: Post, connection: &Conn) -> bool {
    diesel::update(posts::table.find(post.id))
        .set(&post)
        .execute(connection)
        .is_ok()
}

pub fn delete_post(id: i32, connection: &Conn) -> bool {
    diesel::delete(posts::table.find(id))
        .execute(connection)
        .is_ok()
}
