use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ShortenUrl {
    pub id: String,
    pub url: String,
}

#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            db_pool: pool,
        }
    }

    pub async fn add_url(&self, url: ShortenUrl) {
        sqlx::query!("INSERT INTO urls (id, url) VALUES ($1, $2)", url.id, url.url)
            .execute(&self.db_pool)
            .await
            .expect("Failed to insert url");
    }

    pub async fn get_url(&self, id: &str) -> Option<ShortenUrl> {
        sqlx::query!("SELECT id, url FROM urls WHERE id = $1", id)
            .fetch_optional(&self.db_pool)
            .await
            .expect("Failed to fetch url")
            .map(|row| ShortenUrl {
                id: row.id,
                url: row.url,
            })
    }
}