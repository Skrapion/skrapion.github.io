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

use std::collections::{hash_map, HashMap};
use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use base64::{display::Base64Display, engine::general_purpose::STANDARD};
use image::imageops::FilterType;
use tokio::task::JoinHandle;

use crate::config::*;
use crate::utils::*;

pub const SIZES: [u32; 6] = [20, 200, 400, 800, 1200, 1920];

pub struct ThumbnailData {
    pub placeholder: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
pub struct ThumbnailFuturesMap {
    hashmap: HashMap<String, Box<JoinHandle<Result<ThumbnailData>>>>,
}

pub type ThumbnailMap = HashMap<String, ThumbnailData>;

impl ThumbnailFuturesMap {
    pub fn queue_image(&mut self, slug: String, file: String, config: &Config, regenerate: bool) {
        let path = slug.clone() + "/" + &file;
        if let hash_map::Entry::Vacant(entry) = self.hashmap.entry(path) {
            entry.insert(Box::new(tokio::task::spawn(load_images(
                slug,
                file,
                config.post_dir.clone(),
                config.out_dir.clone(),
                regenerate,
            ))));
        }
    }

    pub async fn join_all(&mut self) -> Result<ThumbnailMap> {
        println!("Awaiting image generation");

        let mut thumbnail_map = HashMap::with_capacity(self.hashmap.len());

        for (k, v) in self.hashmap.drain() {
            thumbnail_map.insert(k, v.await??);
        }

        Ok(thumbnail_map)
    }
}

fn scale_image(
    img: &mut Option<image::DynamicImage>,
    in_file: &PathBuf,
    out_file: &PathBuf,
    w: u32,
    h: u32,
    fill: bool,
) {
    if img.is_none() {
        *img = Some(image::open(in_file).unwrap_or_else(|_| {
            panic!(
                "Could not open image file for scaling: {}",
                in_file.display()
            )
        }));
    }

    if let Some(ref image) = img {
        let scaled = match fill {
            true => image.resize_to_fill(w, h, FilterType::CatmullRom),
            false => image.resize(w, h, FilterType::CatmullRom),
        };
        scaled.save(out_file).unwrap();
    }
}

async fn load_images(
    slug: String,
    file: String,
    post_dir: String,
    out_dir: String,
    regenerate: bool,
) -> Result<ThumbnailData> {
    let current_dir = env::current_dir()?;
    let post_dir = current_dir.clone().join(post_dir);
    let out_dir = current_dir.clone().join(out_dir);
    let dir = PathBuf::from(slug);
    let in_file = post_dir.join(dir.clone()).join(&file);
    let basename = in_file.file_stem().unwrap().to_str().unwrap().to_string();

    let out_file = |w: u32| -> PathBuf {
        out_dir
            .join(dir.clone())
            .join(basename.clone() + "-" + &w.to_string() + ".jpg")
    };

    let in_file_modified = fs::metadata(&in_file)
        .unwrap_or_else(|_| panic!("Could not open metadata for file {}", in_file.display()))
        .modified()?;

    let (width, height) = image::image_dimensions(&in_file)?;
    let mut img = Option::<image::DynamicImage>::None;

    for w in SIZES {
        if w < width {
            let out_file = out_file(w);

            if regenerate || should_update_from_time(&in_file_modified, &out_file) {
                println!("Generating {}", out_file.display());
                scale_image(&mut img, &in_file, &out_file, w, u32::MAX, false);
            }
        }
    }

    if width <= SIZES[SIZES.len() - 1] {
        let out_file = out_file(width);

        if regenerate || should_update_from_time(&in_file_modified, &out_file) {
            println!("Copying {}", out_file.display());
            fs::copy(&in_file, out_file)?;
        }
    }

    if file == "cover.jpg" {
        let og_image_file = out_dir.join(dir.clone()).join("ogImage.jpg");
        if regenerate || should_update_from_time(&in_file_modified, &og_image_file) {
            println!("Generating {}", og_image_file.display());
            scale_image(&mut img, &in_file, &og_image_file, 1200, 630, true);
        }
    }

    println!("Placeholder Ready {}", in_file.display());
    let scaled20 = fs::read(out_file(20))?;

    Ok(ThumbnailData {
        placeholder: "data:image/jpeg;base64,".to_string()
            + &Base64Display::new(scaled20.as_slice(), &STANDARD).to_string(),
        width,
        height,
    })
}
