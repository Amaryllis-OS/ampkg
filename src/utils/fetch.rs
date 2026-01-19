use std::{collections::HashMap, error, path::Path};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use futures_util::stream::FuturesUnordered;

use crate::info;

async fn fetch_url_worker<P: AsRef<Path>>(client: &Client, url: String, file: P, pb: ProgressBar) -> Result<(), Box<dyn error::Error>> {
    let response = client.get(url).send().await?;
    let total_size = response.content_length().ok_or("Failed to get content length")?;
    pb.set_length(total_size);

    let mut file = tokio::fs::File::create(file).await?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    return Ok(());
}


pub async fn fetch_url_with_minisign<P: AsRef<Path> + Clone>(
    download_files: HashMap<String, P>, 
    minisign_urls: Vec<String>
) -> Result<(), Box<dyn error::Error>> {
    let client = Client::new();
    let pb = MultiProgress::new();
    let max_parallel_downloads = std::thread::available_parallelism()?.get() / 2;

    let download_dir: &Path = "/tmp/minisign_files".as_ref();
    if !download_dir.exists() {
        tokio::fs::create_dir_all(download_dir).await?;
    }

    info!("Downloading all minisign files...");
    let mut minisign_futures = FuturesUnordered::new();
    
    for (idx, url) in minisign_urls.into_iter().enumerate() {
        let pb = pb.add(ProgressBar::new(0).with_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes}")?
                .progress_chars("#> ")
        ));
        let file_path = download_dir.join(idx.to_string());
        minisign_futures.push(fetch_url_worker(&client, url, file_path, pb));
    }

    while let Some(_) = minisign_futures.next().await {}

    info!("Downloading main files...");
    let mut all_downloads = Vec::new();
    for (url, file_path) in download_files {
        let pb = pb.add(ProgressBar::new(0).with_style(
            ProgressStyle::default_bar()
                .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
                .progress_chars("#> ")
        ));
        all_downloads.push(fetch_url_worker(&client, url, file_path, pb));
    }

    let mut futures = FuturesUnordered::new();
    let mut all_iter = all_downloads.into_iter();

    for _ in 0..max_parallel_downloads {
        if let Some(fut) = all_iter.next() {
            futures.push(fut);
        }
    }

    while let Some(_) = futures.next().await {
        if let Some(fut) = all_iter.next() {
            futures.push(fut);
        }
    }

    Ok(())
}
