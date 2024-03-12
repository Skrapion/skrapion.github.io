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

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::Write;
use std::{env, fs};

use anyhow::Result;
use chrono::NaiveDate;
use handlebars::Handlebars;

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

        for post_data in posts.values_mut() {
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

async fn collect_posts(config: &Config, regenerate: bool) -> Result<CollectedPosts> {
    let post_dir = env::current_dir()?.join(&config.post_dir);
    let out_dir = env::current_dir()?.join(&config.out_dir);

    let mut posts_by_parent_sorted = BTreeMap::<String, BTreeMap<String, PostData>>::new();
    let mut post_titles = BTreeMap::new();
    let mut reverse_tags = BTreeMap::new();
    let mut latest_date = String::new();

    let mut thumbnail_futures_map = ThumbnailFuturesMap::default();

    for entry in fs::read_dir(&post_dir)? {
        let entry = entry?;
        let path = entry.path();

        let post_data = deserialize_md(&path, config)?;
        if let Some(ref postdate) = post_data.postdate {
            if &latest_date < postdate {
                latest_date = postdate.clone();
            }
        }

        if path.is_dir() {
            fs::create_dir_all(&out_dir.clone().join(&post_data.slug))?;
        }

        if let Some(ref title) = post_data.title {
            post_titles.insert(post_data.slug.clone(), title.clone());
        }

        if post_data.parent.is_empty() {
            for tag in &post_data.tags {
                reverse_tags
                    .entry(tag.clone())
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

        if path.is_dir() {
            for pic in &post_data.pics {
                match pic.filetype.as_str() {
                    "pic" => {
                        thumbnail_futures_map.queue_image(
                            post_data.slug.clone(),
                            pic.filename.clone(),
                            config,
                            regenerate,
                        );
                    }
                    "copy" => {
                        let in_file = post_dir
                            .join(post_data.slug.clone())
                            .join(pic.filename.clone());
                        let out_file = out_dir
                            .join(post_data.slug.clone())
                            .join(pic.filename.clone());
                        if regenerate || should_update(&in_file, &out_file)? {
                            println!("Copying {}", out_file.display());
                            fs::copy(in_file, out_file)?;
                        }
                    }
                    _ => {}
                }
            }

            thumbnail_futures_map.queue_image(
                post_data.slug.clone(),
                "cover.jpg".to_string(),
                config,
                regenerate,
            );
        }

        let zero_date = "0000-00-00".to_string();
        let date = post_data.date.as_ref().unwrap_or(&zero_date);
        let key = date.clone() + &post_data.slug;
        posts_by_parent_sorted
            .entry(post_data.parent.clone())
            .or_default()
            .insert(key, post_data);
    }

    let posts_by_parent = collate_posts(&reverse_tags, posts_by_parent_sorted).await?;
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
            let mut slug = String::new();

            // TODO: Reorganize code to be recursive so we don't rely on this
            // hard-coded check anymore.
            if post_data.slug != "index.html" {
                slug = post_data.slug.clone();
            }
            println!("Generating {}", slug);

            let title = match post_data.title {
                Some(ref title) => title.clone() + " - " + &config.site_name,
                None => config.site_name.clone(),
            };

            let mut out_file = out_dir.clone().join(&post_data.slug);
            if out_file.is_dir() {
                out_file.push("index");
                out_file.set_extension(&post_data.extension);
            }

            let thumbnail = match post_data.default_thumbnail {
                true => config.site_thumbnail.clone(),
                false => slug.clone() + "/ogImage.jpg",
            };

            let header_data = HeaderData {
                title: &title,
                description: {
                    if let Some(ref desc) = post_data.description {
                        desc
                    } else {
                        &config.site_description
                    }
                },
                url: &slug,
                thumbnail: &thumbnail,
                cachebust,
            };

            let mut postdate_vec: Posts = vec![];
            let post_with_children = PostWithChildren {
                children: match post_data.posts_by {
                    PostsBy::ByDate => cposts.posts_by_parent.get(&slug),
                    PostsBy::ByPostdate => {
                        let mut postdate_map = BTreeMap::<String, &PostData>::new();
                        for post in &cposts.posts_by_parent[""] {
                            if let Some(ref postdate) = post.postdate {
                                postdate_map.insert(postdate.clone() + &post.slug, post);
                            }
                        }
                        for post in postdate_map.values().rev() {
                            postdate_vec.push((*post).clone());
                        }
                        Some(&postdate_vec)
                    }
                },
                post: Some(post_data),
                header: &header_data,
            };

            handlebars.register_template_string("body", post_data.body.clone())?;
            let rendered = handlebars
                .render(&post_data.template_content, &post_with_children)
                .map_err(|e| format!("{:?}", e))
                .unwrap();

            if !post_data.skip_content {
                let mut content_file = out_file.clone();
                content_file.set_extension("content.".to_string() + &post_data.extension);

                let mut output = File::create(content_file)?;
                write!(output, "{}", rendered)?;
            }

            handlebars.register_template_string("content", rendered)?;
            let output = File::create(out_file)?;
            handlebars
                .render_to_write(&post_data.template_root, &header_data, output)
                .map_err(|e| format!("{:?}", e))
                .unwrap();
        }
    }

    Ok(())
}

pub async fn generate_site(regenerate: bool, config: &Config) -> Result<()> {
    let cposts = collect_posts(config, regenerate).await?;

    let mut handlebars = setup_handlebars(&cposts.post_titles, &cposts.thumbnail_map)?;

    let cachebust = NaiveDate::parse_from_str(&cposts.latest_date, "%F")?
        .signed_duration_since(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
        .num_days()
        .to_string();

    generate_posts(config, &cposts, &mut handlebars, &cachebust).await?;

    Ok(())
}
