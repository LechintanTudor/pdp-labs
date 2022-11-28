#![allow(dead_code)]

mod args;
mod async_client;
mod client;
mod reponse;

use args::Args;
use clap::Parser;

const DOWNLOAD_DIR: &str = "downloads";
const REQUEST_PARAMS: &[(&str, &str, &str)] = &[
    ("193.231.20.34:80", "www.cs.ubbcluj.ro", "/~rlupsa/edu/pdp/"),
    ("193.231.20.34:80", "www.cs.ubbcluj.ro", "/~rlupsa/edu/pdp/lecture-1-intro.html"),
    (
        "193.231.20.34:80",
        "www.cs.ubbcluj.ro",
        "/~rlupsa/edu/pdp/lecture-2-handling-concurrency.html",
    ),
    ("193.231.20.34:80", "www.cs.ubbcluj.ro", "/~rlupsa/edu/pdp/lecture-3-more-concurrency.html"),
    (
        "193.231.20.34:80",
        "www.cs.ubbcluj.ro",
        "/~rlupsa/edu/pdp/lecture-5-futures-continuations.html",
    ),
];

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    std::fs::create_dir_all(DOWNLOAD_DIR)?;

    if args.aio {
        println!("Downloading files in asynchronous mode...");
        main_async()
    } else {
        println!("Downloading files in synchronous mode...");
        main_sync()
    }
}

#[tokio::main]
async fn main_async() -> anyhow::Result<()> {
    use tokio::fs::OpenOptions;
    use tokio::io::AsyncWriteExt;
    use tokio::task::JoinSet;

    let mut requests = JoinSet::<anyhow::Result<()>>::new();

    for (i, (address, host, path)) in REQUEST_PARAMS.iter().enumerate() {
        requests.spawn(async move {
            let response = async_client::send_get_request(address, host, path).await?;

            let path = format!("{DOWNLOAD_DIR}/{i}.html");
            let mut file = OpenOptions::new().write(true).create(true).open(&path).await?;
            file.write_all(response.body().as_bytes()).await?;

            Ok(())
        });
    }

    while let Some(response) = requests.join_next().await {
        match response {
            Ok(Ok(_)) => {
                println!("Task completed successfully!");
            }
            Ok(Err(error)) => {
                println!("Task failed: {error}");
            }
            Err(join_error) => {
                eprintln!("Failed to join task: {join_error}");
            }
        }
    }

    Ok(())
}

fn main_sync() -> anyhow::Result<()> {
    use rayon::prelude::*;
    use std::fs::OpenOptions;
    use std::io::Write;

    let results: Vec<_> = REQUEST_PARAMS
        .par_iter()
        .enumerate()
        .map(|(i, (address, host, path))| -> anyhow::Result<()> {
            let response = client::send_get_request(address, host, path)?;

            let path = format!("{DOWNLOAD_DIR}/{i}.html");
            let mut file = OpenOptions::new().write(true).create(true).open(&path)?;
            file.write_all(response.body().as_bytes())?;

            Ok(())
        })
        .collect();

    for result in results {
        match result {
            Ok(_) => {
                println!("Task completed successfully!");
            }
            Err(error) => {
                println!("Task failed: {error}");
            }
        }
    }

    Ok(())
}
