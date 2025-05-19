mod dto;

use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{DECK_SIZE, DeckProvider, Hand};

const LIST_HANDS_PAGE_SIZE: usize = 5;
const LIST_HANDS_LIMIT: usize = DECK_SIZE - LIST_HANDS_PAGE_SIZE;
const HISTORY_PAGE_SIZE: usize = 10;

use dto::{
    ApiError, CompareHandsRequest, CompareHandsResponse, CreateDeckResponse, HistoryItem,
    HistoryResponse, Json, ListHandsResponse, Pagination,
};

pub async fn create_deck() -> impl IntoResponse {
    Json(CreateDeckResponse { id: Uuid::new_v4() })
}

pub async fn list_hands(
    State(deck_provider): State<Arc<DeckProvider>>,
    State(pool): State<SqlitePool>,
    Path(deck_id): Path<Uuid>,
    Query(Pagination { offset }): Query<Pagination>,
) -> impl IntoResponse {
    if offset > LIST_HANDS_LIMIT {
        return Err(ApiError::UserInput {
            description: format!(
                "Invalid offset. Expected a number between 0 and {LIST_HANDS_LIMIT}, got {}",
                offset
            ),
        });
    }
    let deck = deck_provider.get_with_seed(deck_id);
    let next_offset = offset + 5;

    add_history(&pool, deck_id, offset).await?;

    deck[offset..next_offset]
        .try_into()
        .map(|cards: &[_; 5]| {
            Json(ListHandsResponse {
                hand: Hand::from(cards.clone()),
                next_offset: (next_offset < DECK_SIZE - 5).then_some(next_offset),
            })
        })
        .map_err(|_| ApiError::InternalServer)
}

pub async fn compare_hands(
    Json(CompareHandsRequest { hands }): Json<CompareHandsRequest>,
) -> impl IntoResponse {
    let winner = hands.iter().max_by_key(|h| Hand::from(h.hand.clone()));

    let Some(winner) = winner else {
        return Json(CompareHandsResponse { winners: vec![] });
    };

    let winners = hands
        .iter()
        .filter(|h| Hand::from(h.hand.clone()) == Hand::from(winner.hand.clone()))
        .cloned()
        .collect();

    Json(CompareHandsResponse { winners })
}

pub async fn history(
    Query(pagination): Query<Pagination>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    fetch_history(&pool, pagination.offset, HISTORY_PAGE_SIZE + 1)
        .await
        .map(|mut items| {
            let next_offset =
                (items.len() > HISTORY_PAGE_SIZE).then_some(pagination.offset + HISTORY_PAGE_SIZE);
            items.truncate(HISTORY_PAGE_SIZE);

            Json(HistoryResponse { items, next_offset })
        })
}

async fn add_history(pool: &SqlitePool, deck: Uuid, offset: usize) -> Result<(), ApiError> {
    let now = chrono::Utc::now().timestamp_millis();
    let offset = offset as i64;

    sqlx::query!(
        r#"INSERT INTO history(deck, offset, time)
           VALUES (?, ?, ?)
           ON CONFLICT (deck, offset) DO UPDATE SET time = excluded.time
        "#,
        deck,
        offset,
        now,
    )
    .execute(pool)
    .await
    .map_err(|_| ApiError::InternalServer)?;

    Ok(())
}

async fn fetch_history(
    pool: &SqlitePool,
    offset: usize,
    page_size: usize,
) -> Result<Vec<HistoryItem>, ApiError> {
    let page_size = page_size as i64;
    let offset = offset as i64;
    sqlx::query_as!(
        HistoryItem,
        r#"SELECT offset, time as "time!: u64", deck as "deck!: Uuid"
           FROM history
           ORDER BY time DESC
           LIMIT ? OFFSET ?
        "#,
        page_size,
        offset
    )
    .fetch_all(pool)
    .await
    .map_err(|_| ApiError::InternalServer)
}
