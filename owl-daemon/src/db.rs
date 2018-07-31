use diesel::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

#[derive(Clone)]
pub struct DbPool {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl DbPool {
    pub fn run<T>(&self, f: &Fn(r2d2::PooledConnection<ConnectionManager<PgConnection>>) -> T) -> T {
        f(self.pool.get().unwrap())
    }
}
