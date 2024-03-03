use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use serde::{Serialize, Deserialize};
use crate::credits::credits;

#[derive(Deserialize)]
#[serde(untagged)]
enum PicMetadataIn {
    Basic(String),
    Extended(PicMetadata),
}

#[derive(Deserialize)]
#[derive(Serialize, Default)]
pub struct PicMetadata {
    pub filename: String,
    #[serde(rename="type", default="default_pic")]
    pub filetype: String,
    #[serde(default)]
    pub credit: String,
    #[serde(skip_deserializing)]
    pub fullname: String,
    #[serde(skip_deserializing)]
    pub site: String,
}

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct PostData {
    #[serde(skip_deserializing)]
    pub slug: String,
    pub title: String,
    pub description: String,
    pub date: String,
    #[serde(default)]
    pub postdate: String,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub parent: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(skip_serializing, rename="pics", default="default_cover")]
    pics_in: Vec<PicMetadataIn>,
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

fn default_pic() -> String {
    "pic".to_string()
}

fn default_cover() -> Vec<PicMetadataIn> {
    vec![PicMetadataIn::Basic("cover.jpg".to_string())]
}

pub type Posts = Vec<PostData>;

#[derive(Serialize)]
pub struct HeaderData<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub url: &'a str,
    pub thumbnail: &'a str,
    pub latest_date: &'a str,
}

#[derive(Serialize)]
pub struct PostWithChildren<'a> {
    pub post: Option<&'a PostData>,
    pub children: Option<&'a Posts>,
    pub header: &'a HeaderData<'a>,
}

pub fn deserialize_md(dir: &PathBuf) -> Result<PostData> {
    let slug = dir.file_name().unwrap().to_str().unwrap().to_string();
    println!("Parsing {}", &slug);

    let mut path = dir.clone();
    path.push("index.md");

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut front_matter = String::new();
    let mut content = String::new();
    #[derive(PartialEq)]
    enum Mode { Pre, FrontMatter, Content }
    let mut mode = Mode::Pre;

    for line in reader.lines() {
        let line = line?;
        match mode {
            Mode::Pre => {
                if line != "---" {
                    return Err(anyhow!("File did not start with ---"));
                }
                mode = Mode::FrontMatter;
            },
            Mode::FrontMatter => {
                if line == "---" {
                    mode = Mode::Content;
                } else {
                    front_matter += "\n";
                    front_matter += &line;
                }
            },
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
    post_data.slug = slug;
    if post_data.postdate == "" {
        post_data.postdate = post_data.date.clone();
    }

    for pic in &post_data.pics_in {
        let mut pic_out: PicMetadata = Default::default();
        match pic {
            PicMetadataIn::Basic(filename) => {
                pic_out.filename = filename.to_string();
                pic_out.filetype = "pic".to_string();
            },
            PicMetadataIn::Extended(metadata) => {
                pic_out.filename = metadata.filename.to_string();
                pic_out.filetype = metadata.filetype.to_string();
                pic_out.credit = metadata.credit.to_string();
            }
        }

        let (fullname, site) = credits(&pic_out.credit);
        pic_out.fullname = fullname.to_string();
        pic_out.site = site.to_string();

        post_data.pics.push(pic_out);
    }

    let parser = pulldown_cmark::Parser::new(&content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    post_data.body = html_output;

    Ok(post_data)
}
