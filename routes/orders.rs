use axum::{routing::{post, get, delete}, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/orders", post(add_order))
        .route("/orders/:table_number", get(query_table))
        .route("/orders/:table_number/:item_name", delete(remove_item))
}

async fn add_order() {}
async fn query_table() {}
async fn remove_item() {}

