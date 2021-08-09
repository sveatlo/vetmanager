#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

mod settings;

use sentry_slog::SentryDrain;
use settings::Settings;
use slog::o;
use slog::Drain;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path = env::var("VM_CONFIG_FILE").ok();

    let settings = Settings::new(config_file_path)?;

    let _sentry_guard = sentry::init((
        settings.sentry_dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let drain = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(drain).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let drain = SentryDrain::new(drain);

    let log = slog::Logger::root(drain, o!());

    info!(log, "starting http server"; "http.listen_address" => settings.http.listen_address);

    Ok(())
}
