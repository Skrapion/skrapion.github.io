use std::{env, fs};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use chrono::NaiveDate;
use clap::Parser;

mod serialize;
use serialize::*;

mod templates;
use templates::*;

mod imagegen;
use imagegen::*;

mod utils;
use utils::*;

mod config;
use config::*;

mod webserver;
use webserver::start_server;

const SITENAME: &str = "Firefang";
const DESCRIPTION: &str = "Rick Yorgason's portfolio blog. Everything from traditional woodworking to video game development.";

struct PreprocessVars {
    posts_by_parent: BTreeMap<String, Posts>,
    post_titles: BTreeMap<String, String>,
    thumbnail_map: ThumbnailMap,
    latest_date: String,
}

async fn pre_generate(
    post_dir: &PathBuf, 
    out_dir: &PathBuf, 
    regenerate: bool,
    config: &Config) 
    -> Result<PreprocessVars>
{
    let mut posts_by_parent_sorted = 
        BTreeMap::<String, BTreeMap<String, PostData>>::new();
    let mut post_titles = BTreeMap::new();
    let mut reverse_tags = BTreeMap::new();
    let mut latest_date = String::new();

    let mut thumbnail_futures_map = ThumbnailFuturesMap::default();

    for entry in fs::read_dir(&post_dir)? {
        let entry = entry?;
        let path = entry.path();

        let post_data = deserialize_md(&path, &config)?;
        if &latest_date < &post_data.postdate {
            latest_date = post_data.postdate.clone();
        }

        fs::create_dir_all(&out_dir.clone().join(&post_data.slug))?;

        post_titles.insert(post_data.slug.clone(), post_data.title.clone());

        if post_data.parent == "" {
            for tag in &post_data.tags {
                reverse_tags.entry(tag.clone())
                    .and_modify(|v: &mut BTreeSet<String>| {
                        v.insert(post_data.slug.clone());
                    })
                    .or_insert({
                        let mut v = BTreeSet::new();
                        v.insert(post_data.slug.clone());
                        v
                    });
            }
        }

        for pic in &post_data.pics {
            match pic.filetype.as_str() {
                "pic" => {
                    thumbnail_futures_map.queue_image(
                        post_data.slug.clone(), 
                        pic.filename.clone(),
                        regenerate);
                },
                "copy" => {
                    let in_file = post_dir.join(post_data.slug.clone())
                        .join(pic.filename.clone());
                    let out_file = out_dir.join(post_data.slug.clone())
                        .join(pic.filename.clone());
                    if regenerate || should_update(&in_file, &out_file)? {
                        println!("Copying {}", out_file.display());
                        fs::copy(in_file, out_file)?;
                    }
                },
                _ => {}
            }
        }

        thumbnail_futures_map.queue_image(
            post_data.slug.clone(), "cover.jpg".to_string(), regenerate
        );

        posts_by_parent_sorted.entry(post_data.parent.clone())
            .or_insert(Default::default())
            .insert(post_data.date.clone() + &post_data.slug, post_data);
    }

    let thumbnail_map = thumbnail_futures_map.join_all().await?;
    let mut posts_by_parent = BTreeMap::<String, Posts>::new();

    for (k, mut posts) in posts_by_parent_sorted {
        let mut last_post_opt: Option<&mut PostData> = None;

        for (_, post_data) in &mut posts {
            let mut similars_len = 0;
            let mut similars_all = BTreeSet::new();
            for tag in &post_data.tags {
                let taglen = reverse_tags[tag].len();
                if taglen > similars_len {
                    post_data.similars = BTreeSet::new();
                }

                for slug in &reverse_tags[tag] {
                    if slug != &post_data.slug {
                        similars_all.insert(slug.clone());
                        if taglen > similars_len {
                            post_data.similars.insert(slug.clone());
                        }
                    }
                }
                if taglen > similars_len {
                    post_data.similars_category = tag.clone();
                    similars_len = taglen;
                }
            }

            if similars_len < 5 {
                post_data.similars_category = "similar".to_string();
                post_data.similars = similars_all;
            }


            match last_post_opt {
                None => {}
                Some(ref mut last_post) => {
                    last_post.newer = post_data.slug.clone();
                    post_data.older = last_post.slug.clone();
                }
            }
            last_post_opt = Some(post_data);
        }

        let mut posts_vec = Vec::new();
        while let Some((_, val)) = posts.pop_last() {
            posts_vec.push(val);
        }

        posts_by_parent.insert(k.clone(), posts_vec);
    }

    Ok(PreprocessVars {
        posts_by_parent,
        post_titles,
        thumbnail_map,
        latest_date,
    })
}

async fn generate_site(regenerate: bool, config: &Config) -> Result<()> {
    let current_dir = env::current_dir()?;
    let post_dir = current_dir.clone().join("posts");
    let out_dir = current_dir.join("docs");

    let ppv = pre_generate(&post_dir, &out_dir, regenerate, &config).await?;

    let mut handlebars = setup_handlebars(ppv.post_titles, &ppv.thumbnail_map)?;

    let cachebust = NaiveDate::parse_from_str(&ppv.latest_date, "%F")?
        .signed_duration_since(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        .num_days().to_string();

    println!("Generating posts");
    for (parent, posts) in &ppv.posts_by_parent {
        println!("Parent post: {}", parent);
        for post_data in posts {
            println!("Generating {}", post_data.slug);
            let header_data = HeaderData {
                title: &(post_data.title.clone() + " - " + SITENAME),
                description: &post_data.description,
                url: &post_data.slug,
                thumbnail: &(post_data.slug.clone() + "/ogImage.jpg"),
                cachebust: &cachebust,
            };

            let post_with_children = PostWithChildren {
                children: {
                    match ppv.posts_by_parent.get(&post_data.slug) {
                        None => None,
                        Some(k) => {
                            Some(&k)
                        }
                    }
                },
                post: Some(&post_data),
                header: &header_data,
            };

            let post_dir = out_dir.clone().join(&post_data.slug);

            handlebars.register_template_string("body", post_data.body.clone())?;
            let rendered = handlebars.render("post", &post_with_children)
                .map_err(|e| { format!("{:?}", e )}).unwrap();
            let mut output = File::create(post_dir.clone().join("content.html"))?;
            write!(output, "{}", rendered)?;

            handlebars.register_template_string("content", rendered)?;
            let output = File::create(post_dir.clone().join("index.html"))?;
            handlebars.render_to_write("default", &header_data, output)
                .map_err(|e| { format!("{:?}", e) }).unwrap();
        }
    }

    println!("Generating front page");
    let header_data = HeaderData {
        title: SITENAME,
        description: DESCRIPTION,
        url: "",
        thumbnail: "assets/images/RickHoldingTheWorld.jpg",
        cachebust: &cachebust,
    };
    let post_with_children = PostWithChildren {
        children: Some(&ppv.posts_by_parent[""]),
        post: None,
        header: &header_data,
    };
    let rendered = handlebars.render("frontpage", &post_with_children)
        .map_err(|e| { format!("{:?}", e )}).unwrap();
    let mut output = File::create(out_dir.clone().join("content.html"))?;
    write!(output, "{}", rendered)?;

    handlebars.register_template_string("content", rendered)?;
    let output = File::create(out_dir.clone().join("index.html"))?;
    handlebars.render_to_write("default", &header_data, output)
        .map_err(|e| { format!("{:?}", e) }).unwrap();


    println!("Generating RSS");
    let mut postmap = BTreeMap::new();
    let mut postdate_map = BTreeMap::<String, &PostData>::new();
    for post in &ppv.posts_by_parent[""] {
        postdate_map.insert(post.postdate.clone() + &post.slug, &post);
    }
    let postdate_vec: Vec<&&PostData> = postdate_map.values().rev().collect();
    postmap.insert("posts", &postdate_vec);
    let output = File::create(out_dir.clone().join("rss.xml"))?;
    handlebars.render_to_write("rss", &postmap, output)
        .map_err(|e| { format!("{:?}", e )}).unwrap();

    println!("Generating 404 page");
    let postmap = BTreeMap::<String, String>::new();
    let rendered = handlebars.render("404", &postmap)
        .map_err(|e| { format!("{:?}", e )}).unwrap();

    let header_data = HeaderData {
        title: &("Error 404 - ".to_string() + SITENAME),
        description: DESCRIPTION,
        url: "/404.html",
        thumbnail: "assets/images/RickHoldingTheWorld.jpg",
        cachebust: &cachebust,
    };
    handlebars.register_template_string("content", rendered)?;
    let output = File::create(out_dir.clone().join("404.html"))?;
    handlebars.render_to_write("default", &header_data, output)
        .map_err(|e| { format!("{:?}", e) }).unwrap();


    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to regenerate all the images from scratch
    #[arg(short, long, action)]
    regenerate: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config(&PathBuf::from("config.yaml"))?;
    let args = Args::parse();
    generate_site(args.regenerate, &config).await?;
    start_server().await?;
    Ok(())
}
