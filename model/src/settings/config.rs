use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub database_max_connections: u32,
    pub kafka_topic: String,
    pub kafka_consumer_group: String,
    pub kafka_reset: Option<bool>,
    pub states_rpc_endpoints: Vec<url::Url>,
    pub kafka_settings: HashMap<String, String>,
}

impl Config {
    pub fn new(path: &str) -> Config {
        let mut conf = config::Config::new();
        if std::path::Path::new(path).exists() {
            conf.merge(config::File::with_name(path)).unwrap();
        }
        conf.try_into::<Config>()
            .unwrap_or_else(|e| panic!("Error parsing config: {}", e))
    }
}
