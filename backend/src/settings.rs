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
    pub http: HTTP,
}

#[derive(Default, Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct HTTP {
    pub listen_address: String,
}

impl Settings {
    pub fn new(filepath: Option<String>) -> Result<Self, ConfigError> {
        let mut s = Config::default();

        s.set_default(
            "sentry_dsn",
            "https://52e933ff5fc4441b88037fe772c536a7@o331834.ingest.sentry.io/5882704",
        )?;

        if let Some(filepath) = filepath {
            s.merge(File::with_name(&*filepath))?;
        }
        s.merge(Environment::new().prefix("VM").separator("__"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}

impl Default for HTTP {
    fn default() -> Self {
        Self {
            listen_address: ":1206".into(),
        }
    }
}
