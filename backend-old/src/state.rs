use crate::db::DB;

#[derive(Clone)]
pub struct AppState {
    pub log: slog::Logger,
    pub db: DB,
}

impl AppState {
    pub fn new(log: slog::Logger, db: DB) -> AppState {
        AppState { log, db }
    }
}
