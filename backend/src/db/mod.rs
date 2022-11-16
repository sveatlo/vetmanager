use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::{Pool, PooledConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::error::Error;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

#[derive(Clone)]
pub struct DB {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl DB {
    pub fn new(database_url: &str) -> Self {
        Self::from_pool_builder(database_url, diesel::r2d2::Builder::default())
    }

    pub fn from_pool_builder(
        database_url: &str,
        builder: diesel::r2d2::Builder<ConnectionManager<PgConnection>>,
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

    pub fn run_migrations(&mut self) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
        self.conn().run_pending_migrations(MIGRATIONS)?;

        Ok(())
    }
}
