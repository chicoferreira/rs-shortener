use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ShortenUrl {
    pub id: String,
    pub url: String,
}

pub type AppState = Arc<RwLock<InnerAppState>>;

#[derive(Default)]
pub struct InnerAppState {
    pub urls: HashMap<String, ShortenUrl>,
}