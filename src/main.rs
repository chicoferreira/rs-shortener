use std::env;
use std::net::SocketAddr;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum_macros::debug_handler;
use sqlx::PgPool;
use tracing::{debug, info};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::app::{AppState, ShortenUrl};

mod app;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let pool = PgPool::connect(&env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to Postgres");

    info!("Connected to Postgres");

    let state = AppState::new(pool);

    let router = Router::new()
        .route("/:id", get(redirect_to_url))
        .route("/", post(create_shorten_url))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum_server::bind(addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn create_shorten_url(State(state): State<AppState>, Json(url): Json<ShortenUrl>) -> Json<ShortenUrl> {
    state.add_url(url.clone()).await;
    debug!("Created short url: {:?}", url);
    Json(url)
}

#[debug_handler]
async fn redirect_to_url(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    if let Some(url) = state.get_url(&id).await {
        debug!("Redirecting to url {:?}", url);
        Ok(Redirect::to(url.url.as_str()))
    } else {
        debug!("Url not found for id: {}", id);
        Err(StatusCode::NOT_FOUND)
    }
}