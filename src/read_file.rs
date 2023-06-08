use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub cpu: CPUConfig,
    pub ram: RAMConfig,
    pub disk: DiskConfig,
}

#[derive(Debug, Deserialize)]
pub struct CPUConfig {
    pub enabled: bool,
    pub threshold: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RAMConfig {
    pub enabled: bool,
    pub threshold: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct DiskConfig {
    pub enabled: bool,
    pub threshold: Option<i32>,
    pub paths: Vec<String>,
}

pub fn read_config(file_path: PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(reader)?;

    Ok(config)
}
