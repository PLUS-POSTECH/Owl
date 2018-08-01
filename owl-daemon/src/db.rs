use diesel::{Connection, PgConnection};
use diesel::result::Error;
use r2d2;
use r2d2_diesel::ConnectionManager;

#[derive(Clone)]
pub struct DbPool {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl DbPool {
    pub fn run<T, E>(&self, f: &Fn(&PgConnection) -> Result<T, E>) -> Result<T, E>
        where E: From<Error> {
        let connection = self.pool.get().unwrap();
        connection.transaction::<T, E, _>(|| {
            f(&*connection)
        })
    }
}
