#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod auth;
mod db;
mod repository;
mod response;
mod schema;
mod settings;
mod state;

mod users;

use crate::diesel::ExpressionMethods;
use diesel::query_dsl::methods::FilterDsl;
use sentry_slog::SentryDrain;
use settings::Settings;
use slog::{error, o, Drain};
use state::AppState;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use warp::{Filter, Rejection, Reply};

use crate::response::ErrorResponse;

embed_migrations!("./migrations/");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path = env::var("VM_CONFIG_FILE").ok();
    let settings = Settings::new(config_file_path)?;

    // logging, tracing, etc.
    let log: slog::Logger;
    {
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
        log = slog::Logger::root(drain, o!());
    }

    // create database connection pool
    // let db = db::DB::new(settings.database.url.as_str());
    let db = db::DB::new("postgresql://root@db:26257/postgres?sslmode=disable".into());

    embedded_migrations::run(&db.conn())?;

    // create app state
    let state = AppState::new(log, db);

    // {
    //     let repo = users::UserRepository::new(state.db.conn());
    //     let new_user = users::NewUser {
    //         username: "sveatlo".into(),
    //         email: "svatopluk@vunder.io".into(),
    //         password: "$2y$12$W9LdAOugpdwTAWBWNqJwtuI4LdaI87yLNzxJX2LMkhOUh2nYKt7Ne".into(),
    //         first_name: "Svätopluk".into(),
    //         last_name: "Hanzel".into(),
    //     };
    //     let user: users::User = repo.create(new_user)?;
    //     println!("new user created: {:?}", user);
    //
    //     let user: users::User = repo.find_by_id(user.id)?;
    //     println!("new user found: {:?}", user);
    //
    //     let deleted_user: users::User = repo.delete_by_id(user.id)?;
    //     println!("deleted the new user: {:?}", deleted_user);
    //
    //     let sveatlo_users: users::User = repo.find_by_username("sveatlo".into())?;
    //     println!("sveatlo users: {:?}", sveatlo_users);
    //
    //     return Ok(());
    // }

    // HTTP
    {
        let index = warp::path::end().map(|| response::success("Hello, world"));
        let auth_routes = auth::http::get_routes(state.clone());

        async fn handle_rejection(
            err: Rejection,
        ) -> Result<impl warp::Reply, std::convert::Infallible> {
            if let Some(err_resp) = err.find::<ErrorResponse>() {
                return Ok(err_resp.clone().into_response());
            }

            println!("{:?}", err);

            Ok(warp::reply::json(&"ERROR".to_string()).into_response())
        }

        let routes = index.or(auth_routes).recover(handle_rejection);

        slog::info!(state.log, "starting http server"; "http.listen_address" => &settings.http.listen_address);
        let bind_address: SocketAddr = settings.http.listen_address.parse()?;
        warp::serve(routes).run(bind_address).await;
    }

    Ok(())
}
