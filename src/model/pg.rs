use diesel::prelude::*;
use r2d2;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_postgres::TlsMode;
use postgres::Connection;
use postgres;
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub type PoolPg = r2d2::Pool<PostgresConnectionManager>;

pub fn init_pool() -> PoolPg{
    let config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new(dotenv!("DATABASE_URL"), TlsMode::None).unwrap();
    r2d2::Pool::new(config, manager).expect("postgres db pool")

}

pub struct ConnPg(pub r2d2::PooledConnection<PostgresConnectionManager>);

impl Deref for ConnPg {
    type Target = Connection;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <'a, 'r> FromRequest<'a, 'r> for ConnPg{
    type Error =  ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ConnPg, () > {
        let pool = request.guard::<State<PoolPg>>()?;
        match pool.get(){
            Ok(conn) => Outcome::Success(ConnPg(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

