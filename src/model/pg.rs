use postgres::{Connection, TlsMode};

pub fn get_conn() -> Connection {
    
    Connection::connect(dotenv!("DATABASE_URL"), TlsMode::None).unwrap()
}
