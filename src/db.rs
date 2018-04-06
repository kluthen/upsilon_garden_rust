
use postgres::{Connection, TlsMode};
use config;

/// Provide a DB handle.
pub fn new_handle() -> Connection  {
    Connection::connect(format!("postgres://{}:{}@{}/{}", 
                            config::DB_USER, config::DB_PASSWORD,
                            config::DB_HOST, config::DB_NAME ), TlsMode::None).unwrap() 
}