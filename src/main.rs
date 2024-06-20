use std::net::SocketAddr;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum_macros::debug_handler;
use tracing::debug;
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

    let router = Router::new()
        .route("/:id", get(redirect_to_url))
        .route("/", post(create_shorten_url))
        .with_state(AppState::default());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum_server::bind(addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn create_shorten_url(State(state): State<AppState>, Json(url): Json<ShortenUrl>) -> Json<ShortenUrl> {
    let mut state = state.write().await;
    state.urls.insert(url.id.clone(), url.clone());
    debug!("Created short url: {:?}", url);
    Json(url)
}

#[debug_handler]
async fn redirect_to_url(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let state = state.read().await;
    if let Some(url) = state.urls.get(&id) {
        debug!("Redirecting to url {:?}", url);
        Ok(Redirect::to(url.url.as_str()))
    } else {
        debug!("Url not found for id: {}", id);
        Err(StatusCode::NOT_FOUND)
    }
}