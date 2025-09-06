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

mod config;
mod generate;
mod imagegen;
mod onesignal;
mod serialize;
mod templates;
mod utils;
mod webserver;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use config::*;
use generate::*;
use onesignal::*;
use webserver::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Whether to regenerate all the images from scratch
    #[arg(short, long, action)]
    regenerate: bool,
    /// Schedule the latest post to be sent as a push notification to subscribers
    #[arg(short, long, action)]
    push: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = match load_config(&PathBuf::from("config.yaml")) {
        Ok(config) => config,
        Err(_e) => panic!("Could not load config.yaml")
    };
    let args = Args::parse();

    if args.regenerate || !args.push {
        generate_site(args.regenerate, &config).await?;
    }
    if args.push {
        push_latest_post(&config).await?;
    }
    if !args.push {
        start_server(config).await?;
    }

    Ok(())
}
