use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use std::ops::Deref;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

pub type PoolDsl = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> PoolDsl {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(dotenv!("DATABASE_URL"));
    r2d2::Pool::new(config, manager).expect("diesel db pool")
}

pub struct ConnDsl(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for ConnDsl {
    type Target = PgConnection;
    #[inline(always)]
    fn deref(&self) -> &Self::Target{
        &self.0
    }
}

impl <'a, 'r> FromRequest<'a, 'r> for ConnDsl{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ConnDsl, () > {
        let pool = request.guard::<State<PoolDsl>>()?;
        match pool.get(){
            Ok(conn) => Outcome::Success(ConnDsl(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

