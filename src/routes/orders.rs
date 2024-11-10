use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json,
    Router
};
use rand::Rng;
use serde_json::json;
use std::sync::Arc;
use crate::models::order;
use crate::state::app_state::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/orders", post(add_order))
        .route("/orders/:table_number", get(query_table))
        .route("/orders/:table_number/:item_name", delete(remove_item))
}

async fn add_order(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<order::TableOrder>,
) -> impl IntoResponse {
    let mut rng = rand::thread_rng();

    for item in &payload.items {
        let new_item = order::MenuItem {
            name: item.name.clone(),
            cooking_time: rng.gen_range(5..=15),
        };
        state
            .orders
            .lock()
            .unwrap()
            .entry(payload.table_number.clone())
            .or_default()
            .push(new_item);
    };
    (StatusCode::CREATED, Json("Order added successfully"))
}

async fn query_table(
    State(state): State<Arc<AppState>>,
    Path(table_number): Path<String>,
) -> impl IntoResponse {
    let orders = state.orders.lock().unwrap();
    if let Some(items) = orders.get(&table_number) {
        return (StatusCode::OK, Json(json!(items)));
    }
    (StatusCode::NOT_FOUND, Json(json!({ "error": "Table not found" })))
}

async fn remove_item(
    State(state): State<Arc<AppState>>,
    Path((table_number, item_name)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut orders = state.orders.lock().unwrap();

    if let Some(items) = orders.get_mut(&table_number) {
        let original_len = items.len();
        items.retain(|item| item.name != item_name);

        if items.len() < original_len {
            return (StatusCode::OK, Json(format!("Item '{}' removed successfully for table '{}'", item_name, table_number)));
        } else {
            return (StatusCode::NOT_FOUND, Json(format!("Item '{}' not found for table '{}'", item_name, table_number)));
        }
    }

    (StatusCode::NOT_FOUND, Json(format!("Table '{}' not found", table_number)))
}

