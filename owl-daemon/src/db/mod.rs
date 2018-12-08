use super::error::Error;
use diesel::PgConnection;
use r2d2::{self, Pool};
use r2d2_diesel::ConnectionManager;

pub mod models;
pub mod schema;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn build_connection_pool(database_url: String) -> Result<DbPool, Error> {
    info!(target: "db", "Initializing connection to: {}", &database_url);
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    info!(target: "db", "Connection established");

    Ok(r2d2::Pool::builder().build(manager)?)
}
