use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookMetadata {
    #[serde(skip)]
    pub id: String,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Authors")]
    pub authors: Vec<String>,
    #[serde(rename = "ISBN")]
    pub isbn: Option<String>,
    #[serde(skip)]
    pub page_count: u32,
    #[serde(skip)]
    pub upload_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BookStatus {
    ToRead,
    InProgress,
    Done,
    Next,
}
