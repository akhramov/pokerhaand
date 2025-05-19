use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_database_url")]
    pub database_url: String,
    #[serde(default = "default_address")]
    pub address: String,
}

pub fn from_env() -> eyre::Result<Config> {
    envy::from_env().map_err(Into::into)
}

fn default_address() -> String {
    "0.0.0.0:3000".into()
}

fn default_database_url() -> String {
    "sqlite::memory:".into()
}
