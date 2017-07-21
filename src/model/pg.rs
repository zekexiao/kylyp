use postgres::{Connection, TlsMode};

pub fn get_conn() -> Connection {
    
    Connection::connect("postgres://dbuser:password@localhost:5432/forum", TlsMode::None).unwrap()
}