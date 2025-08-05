
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use serde_yaml;

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub api_key: String,
    pub database_url: String,
    pub port: u16,
    pub log_level: String,
}

pub fn load_configs() -> Result<Configs, Box<dyn std::error::Error>> {
    let file = File::open("configs.yml")?;
    let reader = BufReader::new(file);
    let configs: Configs = serde_yaml::from_reader(reader)?;
    Ok(configs)
}
