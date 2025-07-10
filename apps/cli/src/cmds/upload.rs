use std::path::Path;
use std::io::{self, Write};
use anyhow::{Context, Result};


pub async fn handle_upload(ctx: &core::AppContext, path: String) -> Result<()> {
    let file_path = Path::new(&path);

    // Step 1: Extract text
    println!("📄 Extracting text from PDF...");
    let (raw_text, pages) = core::extract_text(file_path, 3)
        .with_context(|| format!("Failed to extract text from {:?}", file_path))?;

    // Step 2: Infer metadata
    println!("🤖 Inferring metadata...");
    let mut metadata = core::infer_metadata(&raw_text).await
        .context("Failed to infer metadata using GPT")?;
    metadata.page_count = pages;

    // Step 3: Display metadata
    println!("\n🧠 Metadata Inferred:");
    println!("- Title   : {}", metadata.title.as_deref().unwrap_or("[unknown]"));
    println!("- Authors : {}", metadata.authors.join(", "));
    println!("- Pages   : {}", metadata.page_count);
    println!("- ISBN    : {}", metadata.isbn.as_deref().unwrap_or("[unknown]"));

    // Step 4: Confirm with user
    if !prompt_user_confirmation()? {
        println!("❌ Upload canceled.");
        return Ok(());
    }

    // Step 5: Upload file
    println!("📤 Uploading {}...", file_path.display());
    let (id, s3_path) = core::upload_to_s3_async(file_path, &ctx.config.s3_bucket, &ctx.config.s3_prefix).await?;
    println!("✅ Uploaded to: {}", s3_path);

    // Step 6: Finalize and insert metadata
    metadata.id = id;
    metadata.upload_path = s3_path;

    println!("📬 Inserting metadata...");
    core::insert_book(&ctx.db, &metadata).await
        .context("Failed to insert metadata into database")?;

    Ok(())
}

fn prompt_user_confirmation() -> Result<bool> {
    print!("\nSubmit this book to your library? [Y/n] ");
    io::stdout().flush()?; // flush print before input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let confirmed = input.trim().is_empty() || input.trim().eq_ignore_ascii_case("y");
    Ok(confirmed)
}
