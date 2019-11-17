use diesel::prelude::*;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use dotenv::dotenv;
use std::env;
use std::ops::Deref;

use r2d2;
use diesel::r2d2::ConnectionManager;

embed_migrations!("migrations/");

fn database_url() -> String {
    dotenv().ok();
    let dbhost = env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let dbuser = env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let dbname = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let dbpass = env::var("DATABASE_PASS").expect("DATABASE_PASS must be set");
    format!("mysql://{}:{}@{}/{}", dbuser, dbpass, dbhost, dbname)
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url());
    Pool::new(manager)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url()))
}

pub fn perform_migrations(pool: Pool) {
    let conn = pool.get().unwrap();
    embedded_migrations::run(&conn).unwrap_or_else(|_| panic!("Error running migration."));
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
