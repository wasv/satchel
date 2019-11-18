use diesel::MysqlConnection;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::schema::posts;

#[derive(Queryable, AsChangeset, Insertable, Serialize, Deserialize)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
impl Post {
    pub fn create(post: Post, connection: &MysqlConnection) -> Post {
        diesel::insert_into(posts::table)
            .values(&post)
            .execute(connection)
            .expect("Error creating new post");

        posts::table
            .order(posts::id.desc())
            .first(connection)
            .unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Post> {
        posts::table
            .order(posts::id.asc())
            .load::<Post>(connection)
            .unwrap()
    }

    pub fn update(id: i32, post: Post, connection: &MysqlConnection) -> bool {
        diesel::update(posts::table.find(id))
            .set(&post)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(posts::table.find(id))
            .execute(connection)
            .is_ok()
    }
}
