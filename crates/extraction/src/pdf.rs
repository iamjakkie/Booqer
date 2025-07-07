use std::path::Path;
use anyhow::{Context, Result};
use lopdf::{Document, Object};

pub fn extract_text(path: &Path, max_pages: usize) -> Result<(String, u32)> {
    let doc = Document::load(path)
        .with_context(|| format!("Failed to load PDF from {:?}", path))?;

    let mut text = String::new();

    let pages = doc.get_pages(); 
    for (i, (page_number, object_id)) in pages.iter().enumerate() {
        if i >= max_pages {
            break;
        }

        let page_text = doc.extract_text(&[*page_number])
            .unwrap_or_else(|_| String::new());

        text.push_str(&page_text);
        text.push_str("\n--- PAGE BREAK ---\n");
    }

    Ok((text, pages.len() as u32)) 
}
