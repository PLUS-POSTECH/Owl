use diesel::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

#[derive(Clone)]
pub struct DbPool {
    pub pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl DbPool {
    pub fn run<T>(&self, f: &Fn(&PgConnection) -> T) -> T {
        let connection = self.pool.get().unwrap();
        f(&*connection)
    }
}
