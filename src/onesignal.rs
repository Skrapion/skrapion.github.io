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

use std::env;
use std::fs;
use std::str::FromStr;

use anyhow::{bail, Result};
use chrono_tz::Tz;
use cron::Schedule;

use onesignal_rust_api::apis::configuration::Configuration;
use onesignal_rust_api::models::{Notification, StringMap};
use onesignal_rust_api::*;

use crate::config::*;
use crate::serialize::*;

fn create_configuration(onesignal: &OneSignal) -> Box<Configuration> {
    let mut configuration = apis::configuration::Configuration::new();
    configuration.app_key_token = Some(onesignal.app_key_token.clone());
    configuration.user_key_token = Some(onesignal.user_key_token.clone());
    Box::new(configuration)
}

#[derive(Default)]
struct LatestPostData {
    slug: String,
    title: String,
    description: String,
    post_url: String,
    thumbnail_url: String,
}

fn get_latest_post(config: &Config) -> Result<LatestPostData> {
    let post_dir = env::current_dir()?.join(&config.post_dir);
    let mut latest_date = String::new();

    let mut latest_post_data = LatestPostData::default();

    for entry in fs::read_dir(post_dir)? {
        let entry = entry?;
        let path = entry.path();

        let post_data = deserialize_md(&path, config)?;
        if let Some(ref postdate) = post_data.postdate {
            if &latest_date < postdate {
                latest_date = postdate.clone();

                latest_post_data.slug = entry.file_name().to_str().unwrap().to_string();
                latest_post_data.title = match post_data.title {
                    Some(ref title) => title.clone(),
                    None => config.site_name.clone(),
                };
                latest_post_data.description = match post_data.description {
                    Some(ref desc) => desc.clone(),
                    None => config.site_description.clone(),
                };
                latest_post_data.post_url = config.site_url.clone() + "/" + &latest_post_data.slug;
                latest_post_data.thumbnail_url = latest_post_data.post_url.clone() + "/ogImage.jpg";
            }
        }
    }

    Ok(latest_post_data)
}

fn create_notification(
    onesignal: &OneSignal,
    latest_post_data: LatestPostData,
) -> Box<Notification> {
    let mut notification = Notification::new(onesignal.app_id.clone());

    let mut headings_map = StringMap::new();
    headings_map.en = Some(latest_post_data.title.clone());
    notification.headings = Some(Box::new(headings_map));

    let mut contents_map = StringMap::new();
    contents_map.en = Some(latest_post_data.description.clone());
    notification.contents = Some(Box::new(contents_map));

    notification.url = Some(latest_post_data.post_url.clone());
    notification.chrome_web_image = Some(latest_post_data.thumbnail_url.clone());

    let tz: Tz = onesignal.timezone.parse().unwrap();
    let schedule = Schedule::from_str(&onesignal.crontime).unwrap();
    let next = schedule.upcoming(tz).next().unwrap();
    let pubdate = next.format("%F %T UTC%z");

    notification.send_after = Some(pubdate.to_string());
    notification.is_any_web = Some(true);
    notification.included_segments = Some(vec![String::from("Subscribed Users")]);

    Box::new(notification)
}

async fn send_notification(
    configuration: Box<Configuration>,
    notification: Box<Notification>,
) -> Result<()> {
    // Send notification to the server
    let create_notification_response =
        apis::default_api::create_notification(&configuration, *notification).await?;

    // Check the result
    if let Some(id) = create_notification_response.id {
        println!("Created notification id: {}", id);
    }

    Ok(())
}

pub async fn push_latest_post(config: &Config) -> Result<()> {
    if let Some(onesignal) = &config.onesignal {
        let latest_post_data = get_latest_post(config)?;

        let configuration = create_configuration(onesignal);
        let notification = create_notification(onesignal, latest_post_data);

        let title = notification.headings.as_ref().unwrap().en.as_ref().unwrap();
        let send_after = notification.send_after.as_ref().unwrap();

        println!("Scheduling {} to push at {}", title, send_after);

        send_notification(configuration, notification).await?;
    } else {
        bail!("Missing onesignal settings in config.toml");
    }

    Ok(())
}
