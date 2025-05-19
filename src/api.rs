#[cfg(test)]
mod test;
mod v1;

use std::sync::Arc;

use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use sqlx::SqlitePool;

use crate::DeckProvider;

#[derive(Clone, FromRef)]
struct AppState {
    deck_provider: Arc<DeckProvider>,
    pool: SqlitePool,
}

pub fn router(deck_provider: DeckProvider, pool: SqlitePool) -> Router {
    let app_state = AppState {
        deck_provider: Arc::new(deck_provider),
        pool,
    };

    Router::new()
        .route("/api/v1/decks", post(v1::create_deck))
        .route("/api/v1/decks/{deck_id}", get(v1::list_hands))
        .route("/api/v1/history", get(v1::history))
        .route("/api/v1/hands/compare", post(v1::compare_hands))
        .with_state(app_state)
}
