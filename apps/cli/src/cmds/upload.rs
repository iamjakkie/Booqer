use std::path::Path;
use anyhow::Result;

use common::BookMetadata;
use storage::upload_to_s3;
use extraction::extract_text;
use metadata::infer_metadata;

pub async fn handle_upload(path: String) -> Result<()> {
    let file_path = Path::new(&path);

    // 1. Upload to S3
    println!("Uploading {}...", file_path.display());
    // let s3_path = upload_to_s3(file_path)?;
    // println!("✅ Uploaded to: {}", s3_path);

    // 2. Extract text from first N pages
    println!("Extracting text from PDF...");
    let (raw_text, pages) = extract_text(file_path, 3)?; // first 5 pages

    // 3. Infer metadata
    let mut metadata = infer_metadata(&raw_text).await?; 
    metadata.page_count = pages; 
    // metadata.upload_path = s3_path.clone();

    // 4. Show metadata to user
    println!("\n🧠 Metadata Inferred:");
    println!("- Title  : {}", metadata.title.as_deref().unwrap_or("[unknown]"));
    println!("- Authors: {}", metadata.authors.join(", "));
    println!("- Pages  : {}", metadata.page_count);
    println!("- ISBN   : {}", metadata.isbn.as_deref().unwrap_or("[unknown]"));

    // 5. Confirm submission
    println!("\nSubmit this book to your library? [Y/n]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    if input.trim().to_lowercase() == "n" {
        println!("❌ Upload canceled.");
        return Ok(());
    }

    // 6. TODO: Send metadata to backend
    println!("📬 Metadata would now be sent to the backend...");

    Ok(())
}
