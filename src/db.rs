use crate::error::ServerError;
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::{Pool, PooledConnection};
use std::env;

type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
pub type DB = PooledConnection<ConnectionManager<PgConnection>>;

static POOL: ConnectionPool = {
    let url = env::var("DATABASE_URL").expect("Database url not set");
    let manager = ConnectionManager::<PgConnection>::new(url);
    ConnectionPool::new(manager).expect("Failed to create db connection pool")
};

pub fn init() {
    let conn = connection().expect("Failed to get db connection");
}

pub fn connection() -> Result<DB, ServerError> {
    POOL.get()
        .map_err(|e| ServerError::new(500, format!("Failed getting db connection: {}", e)))
}
