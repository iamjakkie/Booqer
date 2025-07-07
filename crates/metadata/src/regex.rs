use common::BookMetadata;
use anyhow::Result;
use regex::Regex;


pub fn infer_metadata(text: &str, page_count: u32) -> BookMetadata {
    let mut lines = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut title = None;
    let mut authors = vec![];
    let mut isbn = None;

    let isbn_re = Regex::new(r"(97(8|9))?\d{9}(\d|X)") 
        .expect("Failed to compile ISBN regex");

    for (i, line) in lines.iter().enumerate() {
        if isbn.is_none() && isbn_re.is_match(line) {
            isbn = Some(isbn_re.find(line).unwrap().as_str().to_string());
        }

        if title.is_none() && line.len() > 30 && !line.contains("Copyright") && !line.ends_with('.') {
            title = Some(line.to_string());

            if let Some(next) = lines.get(i + 1) {
                if next.len() < 60 && next.split_whitespace().count() <= 6 {
                    authors.push(next.to_string());
                }
            }

            break; 
        }
    }

    BookMetadata {
        title,
        authors,
        isbn,
        page_count,
        upload_path: String::new(), // fill in later from upload
    }
}