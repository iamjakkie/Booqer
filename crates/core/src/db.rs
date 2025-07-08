use sqlx::{PgPool};

use crate::models::BookMetadata;

pub async fn insert_book(pool: &PgPool, book: &BookMetadata) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO books (id, title, authors, isbn, page_count, upload_path)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        book.id,
        book.title,
        &book.authors,
        book.isbn,
        book.page_count as i32,
        book.upload_path,
    )
    .execute(pool)
    .await?;

    Ok(())
}
