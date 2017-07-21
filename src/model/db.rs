use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn pg_conn() -> PgConnection {
    let database_url = "postgres://dbuser:password@localhost/forum";
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}