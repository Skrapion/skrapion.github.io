use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credits {
    pub name: String,
    #[serde(default)]
    pub site: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub credits: HashMap<String, Credits>,
}

pub fn load_config(path: &PathBuf) -> Result<Config> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(&mut reader).unwrap();
    Ok(config)
}
