// Copyright 2024 Rick Yorgason
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
//    this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
//    this list of conditions and the following disclaimer in the documentation
//    and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS “AS IS”
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

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

fn posts() -> String {
    "posts".into()
}
fn docs() -> String {
    "docs".into()
}

#[derive(Deserialize)]
pub struct OneSignal {
    pub app_id: String,
    pub app_key_token: String,
    pub user_key_token: String,
    pub timezone: String,
    pub crontime: String,
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "posts")]
    pub post_dir: String,
    #[serde(default = "docs")]
    pub out_dir: String,
    pub site_url: String,
    pub site_name: String,
    pub site_description: String,
    pub site_thumbnail: String,
    pub onesignal: Option<OneSignal>,
    pub credits: HashMap<String, Credits>,
}

pub fn load_config(path: &PathBuf) -> Result<Config> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let config: Config = serde_yaml::from_reader(&mut reader).unwrap();
    Ok(config)
}
