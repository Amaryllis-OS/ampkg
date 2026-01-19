use std::{collections::HashMap, error, path::Path};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use futures_util::stream::FuturesUnordered;

use crate::{info, utils::{paths::{self, get_public_keys}, verify}};

async fn fetch_url_worker<P: AsRef<Path>>(client: &Client, url: String, file: P, pb: ProgressBar, finish_clear: bool) -> Result<(), Box<dyn error::Error>> {
    let file_name = url.split('/').last().ok_or("Invalid URL")?.to_string();
    pb.set_message(format!("{}", file_name));
    
    let response = client.get(&url).send().await
        .map_err(|e| format!("Failed to download {}: {}", file_name, e))?;
    let total_size = response.content_length().ok_or("Failed to get content length")?;
    pb.set_length(total_size);

    let mut file = tokio::fs::File::create(&file).await
        .map_err(|e| format!("Failed to create file {:?}: {}", file.as_ref(), e))?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        pb.set_position(downloaded);
    }

    if finish_clear{
        pb.finish_and_clear();
    } else {
        pb.finish();
    }
    Ok(())
}


async fn verify_and_download_file<P: AsRef<Path>>(
    client: &Client,
    url: String,
    file_path: P,
    signature_dir: std::path::PathBuf,
    pb: ProgressBar,
) -> Result<(), Box<dyn error::Error>> {
    // ダウンロード
    fetch_url_worker(client, url.clone(), file_path.as_ref(), pb, false).await
        .map_err(|e| format!("Download failed for {}: {}", url, e))?;
    
    // 署名ファイル名を取得
    let file_name = url.split('/').last().ok_or("Invalid URL")?;
    let signature_file = signature_dir.join(format!("{}.minisig", file_name));
    
    // 公開鍵確認
    let public_keys = get_public_keys();
    if public_keys.is_empty() {
        return Err("No public keys found in /usr/share/ampkg/keys/".into());
    }
    
    if !signature_file.exists() {
        return Err(format!("Signature file not found: {:?}", signature_file).into());
    }
    
    // 検証
    verify::verify_minisign_signature(
        &public_keys,
        signature_file.clone(),
        file_path.as_ref().to_path_buf(),
    ).await
        .map_err(|e| format!("Verification failed for {:?}: {}", signature_file, e))?;
    
    Ok(())
}

pub async fn fetch_url_with_minisign<P: AsRef<Path>>(
    download_files: HashMap<String, P>, 
    minisign_urls: Vec<String>
) {
    let client = Client::new();
    let pb = MultiProgress::new();
    let max_parallel_downloads = std::thread::available_parallelism().unwrap().get() / 2;

    let download_dir = std::path::PathBuf::from("/tmp/minisign_files");
    if !download_dir.exists() {
        tokio::fs::create_dir_all(&download_dir).await.unwrap();
    }

    // Step 1: Minisignファイルを全部ダウンロード
    info!("Downloading all minisign files...");
    let mut minisign_futures = FuturesUnordered::new();
    
    for url in minisign_urls.into_iter() {
        let file_name = url.split('/').last().ok_or("Invalid URL").unwrap();
        let pb = pb.add(ProgressBar::new(0).with_style(
            ProgressStyle::default_bar()
                .template("{msg} \t[{bar:40.cyan/blue}] {bytes}/{total_bytes}").unwrap()
                .progress_chars("#> ")
        ));
        let file_path = download_dir.join(file_name);
        minisign_futures.push(fetch_url_worker(&client, url, file_path, pb, true));
    }

    // すべてのminisignダウンロード完了を待機＆エラーチェック
    while let Some(result) = minisign_futures.next().await {
        if let Err(e) = result {
            eprintln!("Error downloading minisign file: {}", e);
        }
    }

    // Step 2: メインファイルをダウンロード＆検証
    info!("Downloading and verifying packages...");
    let mut all_downloads = Vec::new();
    for (url, file_path) in download_files {
        let pb = pb.add(ProgressBar::new(0).with_style(
            ProgressStyle::default_bar()
                .template("{msg} \t[{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})").unwrap()
                .progress_chars("#> ")
        ));
        let download_dir_clone = download_dir.clone();
        all_downloads.push(verify_and_download_file(&client, url, file_path, download_dir_clone, pb));
    }

    let mut futures = FuturesUnordered::new();
    let mut all_iter = all_downloads.into_iter();

    // 並行数制限を適用
    for _ in 0..max_parallel_downloads {
        if let Some(fut) = all_iter.next() {
            futures.push(fut);
        }
    }

    // 1つ完了したら次を追加＆エラーチェック
    while let Some(result) = futures.next().await {
        if let Err(e) = result {
            eprintln!("Error downloading/verifying file: {}", e);
        }
        if let Some(fut) = all_iter.next() {
            futures.push(fut);
        }
    }

}
