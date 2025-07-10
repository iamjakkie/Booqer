use sqlx::PgPool;
use aws_sdk_s3::Client as S3Client;
use anyhow::Result;

use crate::config::AppConfig;

pub struct AppContext {
    pub db: PgPool,
    pub s3: S3Client,
    pub config: AppConfig,
}

impl AppContext {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let db = PgPool::connect(&config.database_url).await?;
        let aws_cfg = aws_config::load_from_env().await;
        let s3 = S3Client::new(&aws_cfg);

        Ok(AppContext { db, s3, config })
    }

    pub async fn close(self) -> Result<()> {
        self.db.close().await;
        Ok(())
    }
}