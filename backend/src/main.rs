mod settings;

use settings::Settings;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let config_file_path = env::var("VM_CONFIG_FILE").ok();
    let settings = Settings::new(config_file_path)?;

    // instrumentation - loggin, tracing, etc.
    let _sentry_guard;
    {
        _sentry_guard = sentry::init((
            settings.sentry_dsn,
            sentry::ClientOptions {
                release: sentry::release_name!(),
                ..Default::default()
            },
        ));

        tracing_subscriber::registry()
            .with(tracing_subscriber::filter::LevelFilter::from_level(
                if settings.debug {
                    tracing::Level::DEBUG
                } else {
                    tracing::Level::INFO
                },
            ))
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG")
                    .unwrap_or_else(|_| "vetmanager_backend=debug,tower_http=debug".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .with(sentry_tracing::layer())
            .init();
    }

    // DB
    let mut db;
    {
        tracing::debug!("connecting to db @ {}...", settings.database.url);
        db = vetmanager_backend::db::DB::new(&settings.database.url);
        db.run_migrations()?;
    }

    let router = vetmanager_backend::create_router();

    let addr = settings.http.listen_address.parse::<SocketAddr>()?;
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("BUG: server failed");

    Ok(())
}
