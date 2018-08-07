use std::env;

use super::error::Error;
use diesel::PgConnection;
use r2d2::{self, Pool};
use r2d2_diesel::ConnectionManager;

pub mod models;
pub mod schema;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn build_connection_pool(database_url: String) -> Result<DbPool, Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Ok(r2d2::Pool::builder().build(manager)?)
}
