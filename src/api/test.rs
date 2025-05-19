use super::*;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{Value, json};
use tower::util::ServiceExt; // for oneshot
use uuid::Uuid;

async fn setup_router() -> Router {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    let deck_provider = Default::default();
    router(deck_provider, pool)
}

async fn make_request(
    app: &Router,
    method: &str,
    uri: &str,
    json_body: Option<&Value>,
) -> (StatusCode, Value) {
    let body = match json_body {
        Some(val) => Body::from(val.to_string()),
        None => Body::empty(),
    };

    let mut builder = Request::builder().method(method).uri(uri);

    if json_body.is_some() {
        builder = builder.header("Content-Type", "application/json");
    }

    let request = builder.body(body).unwrap();
    let response = app.clone().oneshot(request).await.unwrap();

    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&bytes).expect("Invalid JSON response");

    (status, json)
}

#[tokio::test]
async fn test_create_deck_returns_uuid() {
    let app = setup_router().await;
    let (_, json) = make_request(&app, "POST", "/api/v1/decks", None).await;

    assert!(json.get("id").is_some());
    let id_str = json.get("id").unwrap().as_str().unwrap();
    assert!(Uuid::parse_str(id_str).is_ok());
}

#[tokio::test]
async fn test_list_hands_returns_five_cards_and_next_offset() {
    let app = setup_router().await;
    let uri = "/api/v1/decks/3b783e86-9390-495a-8cd0-e5a9a93032c0?offset=0";
    let (_, json) = make_request(&app, "GET", uri, None).await;

    let hand = json.get("hand").unwrap();

    let cards = hand.get("cards").unwrap().as_array().unwrap();

    let ranking = hand.get("ranking_category").unwrap().as_str().unwrap();

    assert_eq!(cards.len(), 5);
    assert_eq!(ranking, "StraightFlush");

    let next_offset = json.get("next_offset").unwrap();

    assert_eq!(next_offset.as_u64().unwrap(), 5);
}

#[tokio::test]
async fn test_list_hands_invalid_offset_returns_error() {
    let app = setup_router().await;
    let deck_id = Uuid::new_v4();
    let uri = format!("/api/v1/decks/{deck_id}?offset=1000");
    let (status, json) = make_request(&app, "GET", &uri, None).await;

    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    let desc = json.get("description").unwrap().as_str().unwrap();

    assert!(desc.contains("Invalid offset"));
}

#[tokio::test]
async fn test_fetch_all_hands_pagination() {
    let app = setup_router().await;
    let deck_id = Uuid::new_v4();
    let mut offset = 0usize;
    let mut all_cards = Vec::new();

    loop {
        let uri = format!("/api/v1/decks/{deck_id}?offset={offset}");
        let (_, json) = make_request(&app, "GET", &uri, None).await;

        let cards = json
            .get("hand")
            .unwrap()
            .get("cards")
            .unwrap()
            .as_array()
            .unwrap()
            .clone();
        all_cards.extend(cards);

        if let Some(next_offset) = json.get("next_offset").and_then(|v| v.as_u64()) {
            offset = next_offset as usize;
        } else {
            break;
        }
    }

    assert_eq!(all_cards.len(), 50);
}

#[tokio::test]
async fn test_compare_hands_single_winner() {
    let app = setup_router().await;
    let payload = json!({
        "hands": [
            { "external_id": "a", "hand": ["ah", "kh", "qh", "jh", "th"] },
            { "external_id": "b", "hand": ["2k", "3k", "4k", "5k", "6k"] }
        ]
    });

    let (status, json) = make_request(&app, "POST", "/api/v1/hands/compare", Some(&payload)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        json["winners"],
        json!([{ "external_id": "a", "hand": ["ah", "kh", "qh", "jh", "th"] }])
    );
}

#[tokio::test]
async fn test_compare_hands_tie_between_two_winners() {
    let app = setup_router().await;

    let payload = serde_json::json!({
        "hands": [
            { "external_id": "a", "hand": ["ah", "kh", "qh", "jh", "th"] }, // Royal Flush
            { "external_id": "b", "hand": ["as", "ks", "qs", "js", "ts"] }, // Royal Flush same rank, different suits
            { "external_id": "c", "hand": ["2k", "3k", "4k", "5k", "6k"] }
        ]
    });

    let (status, json) = make_request(&app, "POST", "/api/v1/hands/compare", Some(&payload)).await;

    assert_eq!(status, StatusCode::OK);

    // Expect two winners (tie)
    let winners = json["winners"].as_array().unwrap();
    assert_eq!(winners.len(), 2);

    let external_ids: Vec<_> = winners
        .iter()
        .map(|w| w["external_id"].as_str().unwrap())
        .collect();
    assert!(external_ids.contains(&"a"));
    assert!(external_ids.contains(&"b"));
}

#[tokio::test]
async fn test_compare_hands_empty_hands_list_returns_empty_winners() {
    let app = setup_router().await;

    let payload = serde_json::json!({ "hands": [] });

    let (status, json) = make_request(&app, "POST", "/api/v1/hands/compare", Some(&payload)).await;

    assert_eq!(status, StatusCode::OK);

    assert!(json["winners"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn test_compare_hands_invalid_hand_format_returns_error() {
    let app = setup_router().await;

    // Hand with fewer than 5 cards
    let payload = serde_json::json!({
        "hands": [
            { "external_id": "a", "hand": ["ah", "kh", "qh"] }
        ]
    });

    let (status, json) = make_request(&app, "POST", "/api/v1/hands/compare", Some(&payload)).await;

    let desc = json.get("description").unwrap().as_str().unwrap();
    assert!(desc.contains("invalid length 3, expected an array of length 5"));
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_history_returns_empty_when_no_entries_exist() {
    let app = setup_router().await;

    let (status, json) = make_request(&app, "GET", "/api/v1/history?offset=0", None).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(json["items"].as_array().unwrap().len(), 0);
    assert!(json["next_offset"].is_null());
}

#[tokio::test]
async fn test_history_returns_entries_after_list_hands_calls() {
    let app = setup_router().await;
    let deck_id = Uuid::new_v4();

    // Gjør noen kall som skriver til history via list_hands
    for offset in [0, 5, 10] {
        let uri = format!("/api/v1/decks/{deck_id}?offset={offset}");
        let (_status, _json) = make_request(&app, "GET", &uri, None).await;
    }

    // Nå skal history returnere 3 entries
    let (status, json) = make_request(&app, "GET", "/api/v1/history?offset=0", None).await;

    assert_eq!(status, StatusCode::OK);

    let items = json["items"].as_array().unwrap();
    assert_eq!(items.len(), 3);

    let seen_offsets: Vec<_> = items
        .iter()
        .map(|item| item["offset"].as_u64().unwrap())
        .collect();
    assert!(seen_offsets.contains(&0));
    assert!(seen_offsets.contains(&5));
    assert!(seen_offsets.contains(&10));
}

#[tokio::test]
async fn test_history_pagination_next_offset_set_correctly() {
    let app = setup_router().await;

    // 18 kall --> skal føre til next_offset
    for deck_id in [Uuid::new_v4(), Uuid::new_v4()] {
        for offset in (0..45).step_by(5) {
            let uri = format!("/api/v1/decks/{deck_id}?offset={offset}");
            let (_status, _json) = make_request(&app, "GET", &uri, None).await;
        }
    }

    let (status, json) = make_request(&app, "GET", "/api/v1/history?offset=0", None).await;

    assert_eq!(status, StatusCode::OK);

    let items = json["items"].as_array().unwrap();
    assert_eq!(items.len(), 10); // Page size
    assert_eq!(json["next_offset"].as_u64().unwrap(), 10);
}

#[tokio::test]
async fn test_history_overwrites_existing_entry_with_new_time() {
    let app = setup_router().await;
    let deck_id = Uuid::new_v4();

    // Første kall til offset 0
    let uri = format!("/api/v1/decks/{deck_id}?offset=0");
    let _ = make_request(&app, "GET", &uri, None).await;

    // Hent første timestamp
    let (_, json1) = make_request(&app, "GET", "/api/v1/history?offset=0", None).await;
    let first_time = json1["items"][0]["time"].as_u64().unwrap();

    // Vent litt, gjør nytt kall til samme offset
    tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    let _ = make_request(&app, "GET", &uri, None).await;

    // Hent igjen, skal ha samme deck+offset, men nyere tid
    let (_, json2) = make_request(&app, "GET", "/api/v1/history?offset=0", None).await;
    let second_time = json2["items"][0]["time"].as_u64().unwrap();

    assert_eq!(json2["items"].as_array().unwrap().len(), 1);
    assert!(
        second_time > first_time,
        "Expected time to be updated on second visit"
    );
}
