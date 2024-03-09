use std::str::FromStr;
use std::{env, fs};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use anyhow::Result;
use chrono::NaiveDate;
use handlebars::Handlebars;
use serde::Deserialize;

use crate::config::*;
use crate::imagegen::*;
use crate::serialize::*;
use crate::templates::*;
use crate::utils::*;

struct CollectedPosts {
    posts_by_parent: BTreeMap<String, Posts>,
    post_titles: BTreeMap<String, String>,
    thumbnail_map: ThumbnailMap,
    latest_date: String,
}

async fn collate_posts<'a>(
    reverse_tags: &BTreeMap<String, BTreeSet<String>>,
    posts_by_parent_sorted: BTreeMap<String, BTreeMap<String, PostData>>,
) -> Result<BTreeMap<String, Posts>> {
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

    Ok(posts_by_parent)
}


async fn collect_posts(
    config: &Config, regenerate: bool,
) -> Result<CollectedPosts> {
    let post_dir = env::current_dir()?.join(&config.post_dir);
    let out_dir = env::current_dir()?.join(&config.out_dir);

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
                        &config,
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
            post_data.slug.clone(), "cover.jpg".to_string(), &config, regenerate
        );

        posts_by_parent_sorted.entry(post_data.parent.clone())
            .or_insert(Default::default())
            .insert(post_data.date.clone() + &post_data.slug, post_data);
    }

    let posts_by_parent = 
        collate_posts(&reverse_tags, posts_by_parent_sorted).await?;
    let thumbnail_map = thumbnail_futures_map.join_all().await?;

    Ok(CollectedPosts {
        posts_by_parent,
        post_titles,
        thumbnail_map,
        latest_date,
    })
}

async fn generate_posts(
    config: &Config,
    cposts: &CollectedPosts, 
    handlebars: &mut Handlebars<'_>,
    cachebust: &str,
) -> Result<()> {
    let out_dir = env::current_dir()?.join(&config.out_dir);

    println!("Generating posts");
    for (parent, posts) in &cposts.posts_by_parent {
        println!("Parent post: {}", parent);
        for post_data in posts {
            println!("Generating {}", post_data.slug);
            let header_data = HeaderData {
                title: &(post_data.title.clone() + " - " + &config.site_name),
                description: &post_data.description,
                url: &post_data.slug,
                thumbnail: &(post_data.slug.clone() + "/ogImage.jpg"),
                cachebust: &cachebust,
            };

            let post_with_children = PostWithChildren {
                children: {
                    match cposts.posts_by_parent.get(&post_data.slug) {
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

    Ok(())
}

#[derive(Deserialize)]
struct PageData {
    content: String,
    #[serde(default="default_default")]
    template: String,
    title: Option<String>,
    description: Option<String>,
}

fn default_default() -> String { "default".to_string() }

fn generate_pages(
    config: &Config,
    cposts: &CollectedPosts, 
    handlebars: &mut Handlebars<'_>,
    cachebust: &str,
    dir: &PathBuf,
) -> Result<()> {
    let out_dir = env::current_dir()?.join(&config.out_dir).join(dir);
    let page_dir = env::current_dir()?.join(&config.page_dir).join(dir);

    fs::create_dir_all(&out_dir)?;

    for entry in fs::read_dir(&page_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            generate_pages(
                config, cposts, handlebars, cachebust, 
                &dir.clone().join(entry.file_name()))?
        } else if let Some(ext) = &path.extension() {
            if ext.to_str().unwrap() == "yaml" {
                let mut full_url = dir.clone();
                if entry.file_name() != "index.yaml" {
                    full_url.push(&entry.file_name());
                    full_url.set_extension("html");
                }

                let mut content_url = dir.clone().join(&entry.file_name());
                content_url.set_extension("content.html");

                let mut full_path = 
                    PathBuf::from_str(&config.out_dir)?.join(&full_url);
                if entry.file_name() == "index.yaml" {
                    full_path.push(&entry.file_name());
                    full_path.set_extension("html");
                }
                let content_path = 
                    PathBuf::from_str(&config.out_dir)?.join(&content_url);

                println!("Generating {}", full_path.display());

                let file = File::open(path)?;
                let reader = BufReader::new(file);
                let page_data: PageData = 
                    serde_yaml::from_reader(reader).unwrap();

                let header_data = HeaderData {
                    title: 
                        page_data.title.as_ref().unwrap_or(&config.site_name),
                    description: 
                        page_data.description.as_ref().unwrap_or(&config.site_description),
                    url: 
                        full_url.to_str().expect("Could not convert path to string"),
                    thumbnail: 
                        page_data.description.as_ref().unwrap_or(&config.site_thumbnail),
                    cachebust: &cachebust,
                };
                let post_with_children = PostWithChildren {
                    children: Some(&cposts.posts_by_parent[""]),
                    post: None,
                    header: &header_data,
                };

                let content = page_dir.clone().join(&page_data.content).to_str()
                    .expect(&format!("Could not convert {}/{} to a file",
                                    dir.display(), page_data.content))
                    .to_string();

                handlebars.register_template_file(&content, &content)?;

                let rendered = handlebars.render(&content, &post_with_children)
                    .map_err(|e| { format!("{:?}", e )}).unwrap();

                let mut output = File::create(content_path)?;
                write!(output, "{}", rendered)?;

                handlebars.register_template_string("content", rendered)?;
                let output = File::create(full_path)?;
                handlebars.render_to_write(&page_data.template, &header_data, output)
                    .map_err(|e| { format!("{:?}", e) }).unwrap();
            }
        }
    }

    Ok(())
}

async fn generate_rss(
    config: &Config,
    cposts: &CollectedPosts, 
    handlebars: &mut Handlebars<'_>,
) -> Result<()> {
    let out_dir = env::current_dir()?.join(&config.out_dir);

    println!("Generating RSS");
    let mut postmap = BTreeMap::new();
    let mut postdate_map = BTreeMap::<String, &PostData>::new();
    for post in &cposts.posts_by_parent[""] {
        postdate_map.insert(post.postdate.clone() + &post.slug, &post);
    }
    let postdate_vec: Vec<&&PostData> = postdate_map.values().rev().collect();
    postmap.insert(config.post_dir.clone(), &postdate_vec);
    let output = File::create(out_dir.clone().join("rss.xml"))?;
    handlebars.render_to_write("rss", &postmap, output)
        .map_err(|e| { format!("{:?}", e )}).unwrap();

    Ok(())
}

pub async fn generate_site(regenerate: bool, config: &Config) -> Result<()> {
    let cposts = collect_posts(&config, regenerate).await?;

    let mut handlebars = setup_handlebars(&cposts.post_titles, 
                                          &cposts.thumbnail_map)?;

    let cachebust = NaiveDate::parse_from_str(&cposts.latest_date, "%F")?
        .signed_duration_since(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        .num_days().to_string();

    generate_posts(&config, &cposts, &mut handlebars, &cachebust).await?;
    generate_pages(&config, &cposts, &mut handlebars, &cachebust, &"".into())?;
    generate_rss(&config, &cposts, &mut handlebars).await?;

    Ok(())
}
