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
    let config = load_config(&PathBuf::from("config.yaml"))?;
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
