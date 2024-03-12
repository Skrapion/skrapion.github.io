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

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::config::*;

#[derive(Deserialize, Clone)]
#[serde(untagged)]
enum PicMetadataIn {
    Basic(String),
    Extended(PicMetadata),
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct PicMetadata {
    pub filename: String,
    #[serde(rename = "type", default = "default_pic")]
    pub filetype: String,
    #[serde(default)]
    pub credit: String,
    #[serde(skip_deserializing)]
    pub fullname: String,
    #[serde(skip_deserializing)]
    pub site: String,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub enum PostsBy {
    #[default]
    #[serde(rename = "by_date")]
    ByDate,
    #[serde(rename = "by_postdate")]
    ByPostdate,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PostData {
    // Common params for typical posts
    pub title: Option<String>,
    pub description: Option<String>,
    pub date: Option<String>,
    pub postdate: Option<String>,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub parent: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing, rename = "pics", default = "default_cover_vec")]
    pics_in: Vec<PicMetadataIn>,
    #[serde(default = "default_cover")]
    pub og_image: String,

    // Uncommon params mostly for pages
    #[serde(default = "default_post")]
    pub template_content: String,
    #[serde(default = "default_default")]
    pub template_root: String,
    #[serde(default)]
    pub default_thumbnail: bool,
    #[serde(default)]
    pub skip_content: bool,
    #[serde(default)]
    pub posts_by: PostsBy,
    #[serde(default = "default_html")]
    pub extension: String,

    // Generated after load
    #[serde(skip_deserializing)]
    pub slug: String,
    #[serde(skip_deserializing)]
    pub pics: Vec<PicMetadata>,
    #[serde(skip_deserializing)]
    pub body: String,
    #[serde(skip_deserializing)]
    pub older: String,
    #[serde(skip_deserializing)]
    pub newer: String,
    #[serde(skip_deserializing)]
    pub similars_category: String,
    #[serde(skip_deserializing)]
    pub similars: BTreeSet<String>,
}

fn default_post() -> String {
    "post".to_string()
}

fn default_default() -> String {
    "default".to_string()
}

fn default_pic() -> String {
    "pic".to_string()
}

fn default_html() -> String {
    "html".to_string()
}

fn default_cover() -> String {
    "cover".to_string()
}

fn default_cover_vec() -> Vec<PicMetadataIn> {
    vec![PicMetadataIn::Basic("cover.jpg".to_string())]
}

pub type Posts = Vec<PostData>;

#[derive(Serialize)]
pub struct HeaderData<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub url: &'a str,
    pub thumbnail: &'a str,
    pub cachebust: &'a str,
}

#[derive(Serialize)]
pub struct PostWithChildren<'a> {
    pub post: Option<&'a PostData>,
    pub children: Option<&'a Posts>,
    pub header: &'a HeaderData<'a>,
}

pub fn deserialize_md(dir: &Path, config: &Config) -> Result<PostData> {
    let mut path = dir.to_path_buf();
    if path.is_dir() {
        path.push("index.md");
    }

    println!("Parsing {}", &path.display());

    let file = File::open(&path)
        .unwrap_or_else(|_| panic!("Could not open md file at {}", path.display()));

    let reader = BufReader::new(file);
    let mut front_matter = String::new();
    let mut content = String::new();
    #[derive(PartialEq)]
    enum Mode {
        Pre,
        FrontMatter,
        Content,
    }
    let mut mode = Mode::Pre;

    for line in reader.lines() {
        let line = line?;
        match mode {
            Mode::Pre => {
                if line != "---" {
                    return Err(anyhow!("File did not start with ---"));
                }
                mode = Mode::FrontMatter;
            }
            Mode::FrontMatter => {
                if line == "---" {
                    mode = Mode::Content;
                } else {
                    front_matter += "\n";
                    front_matter += &line;
                }
            }
            Mode::Content => {
                content += &line;
                content += "\n";
            }
        }
    }

    if mode != Mode::Content {
        return Err(anyhow!("File did not contain content or --- was malformed"));
    }

    let mut post_data: PostData = serde_yaml::from_str(&front_matter).unwrap();

    let mut slug = dir.to_path_buf();
    if dir.extension().is_some() {
        slug.set_extension(&post_data.extension);
    }
    let slug = slug.file_name().unwrap().to_str().unwrap().to_string();
    post_data.slug = slug;
    if post_data.postdate.is_none() {
        post_data.postdate = post_data.date.clone();
    }

    for pic in &post_data.pics_in {
        let mut pic_out: PicMetadata = Default::default();
        match pic {
            PicMetadataIn::Basic(filename) => {
                pic_out.filename = filename.to_string();
                pic_out.filetype = "pic".to_string();
            }
            PicMetadataIn::Extended(metadata) => {
                pic_out.filename = metadata.filename.to_string();
                pic_out.filetype = metadata.filetype.to_string();
                pic_out.credit = metadata.credit.to_string();
            }
        }

        if let Some(config) = config.credits.get(&pic_out.credit) {
            pic_out.fullname = config.name.clone();
            pic_out.site = config.site.clone();
        } else {
            pic_out.fullname = pic_out.credit.clone();
        }

        post_data.pics.push(pic_out);
    }

    let parser = pulldown_cmark::Parser::new(&content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    post_data.body = html_output;

    Ok(post_data)
}
