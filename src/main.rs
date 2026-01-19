use std::collections::HashMap;

mod install;
mod utils;


#[tokio::main]
async fn main() {
    // Test
    let mut download_files = HashMap::new();

    download_files.insert(
        "https://ftp.konoha.rest/amaryllis/packages/base-1.0.0~amary1.amp".to_string(),
        "/tmp/base-1.0.0~amary1.amp", 
    );

    let minisign_urls = vec![
        "https://ftp.konoha.rest/amaryllis/packages/base-1.0.0~amary1.amp.minisig".to_string(),
    ];

    utils::fetch::fetch_url_with_minisign(download_files, minisign_urls).await;
}
