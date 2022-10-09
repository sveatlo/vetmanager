use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::{Pool, PooledConnection};

#[derive(Clone)]
pub struct DB {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl DB {
    pub fn new(database_url: &str) -> Self {
        Self::from_pool_builder(database_url, r2d2::Builder::default())
    }

    pub fn from_pool_builder(
        database_url: &str,
        builder: r2d2::Builder<ConnectionManager<PgConnection>>,
    ) -> Self {
        let manager = ConnectionManager::new(database_url);
        let connection_pool = builder
            .build(manager)
            .expect("could not initiate test db pool");
        DB { connection_pool }
    }

    pub fn conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.connection_pool.get().unwrap()
    }
}
