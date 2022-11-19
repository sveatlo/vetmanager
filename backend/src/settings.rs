use config::{Config, ConfigError, Environment, File};

use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub sentry_dsn: String,
    #[serde(default)]
    pub database: Database,
    #[serde(default)]
    pub http: Http,
    #[serde(default)]
    pub auth: Auth,
}

#[derive(Default, Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Http {
    pub listen_address: String,
}

#[derive(Default, Debug, Deserialize)]
pub struct Auth {
    pub jwt_secret: String,
    pub hash_salt: String,
}

impl Settings {
    pub fn new(filepath: Option<String>) -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        builder = builder
            .set_default(
                "sentry_dsn",
                "https://52e933ff5fc4441b88037fe772c536a7@o331834.ingest.sentry.io/5882704",
            )?
            .set_default(
                "database.url",
                "postgresql://root@db:26257/postgres?sslmode=disable",
            )?;
        if let Some(filepath) = filepath {
            builder = builder.add_source(File::with_name(filepath.as_str()));
        }

        builder = builder.add_source(Environment::default().prefix("VM").separator("__"));

        builder.build()?.try_deserialize()
    }
}

impl Default for Http {
    fn default() -> Self {
        Self {
            listen_address: "0.0.0.0:1210".into(),
        }
    }
}
