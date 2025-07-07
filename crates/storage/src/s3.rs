use std::{fs::File, path::Path, io::{Read, BufReader}};
use anyhow::Result;
use aws_sdk_s3::{primitives::ByteStream, Client};
use sha2::{Sha256, Digest};
use hex::encode;

const BUCKET: &str = "booqer";
const PREFIX: &str = "books/";

/// Blocking wrapper for CLI use
pub fn upload_to_s3(path: &Path) -> Result<String> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(upload_to_s3_async(path))
}

async fn upload_to_s3_async(path: &Path) -> Result<String> {
    // Hash file contents
    let mut reader = BufReader::new(File::open(path)?);
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 4096];
    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    let hash = encode(hasher.finalize());
    let key = format!("{PREFIX}{hash}.pdf");

    // Upload to S3 using AWS SDK
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let body = ByteStream::from_path(path.to_path_buf()).await?;

    client.put_object()
        .bucket(BUCKET)
        .key(&key)
        .body(body)
        .send()
        .await?;

    Ok(format!("s3://{}/{}", BUCKET, key))
}
