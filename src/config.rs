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

fn posts() -> String { "posts".into() }
fn docs() -> String { "docs".into() }

#[derive(Deserialize)]
pub struct Config {
    #[serde(default="posts")]
    pub post_dir: String,
    #[serde(default="docs")]
    pub out_dir: String,
    pub site_name: String,
    pub site_description: String,
    pub site_thumbnail: String,
    pub credits: HashMap<String, Credits>,
}

pub fn load_config(path: &PathBuf) -> Result<Config> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(&mut reader).unwrap();
    Ok(config)
}
