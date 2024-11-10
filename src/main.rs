use std::sync::Arc;
use crate::routes::orders;
use crate::state::app_state::AppState;

mod models;
mod routes;
mod state;
mod tests;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let state = Arc::new(AppState::default());

    let router = orders::routes()
        .with_state(state);

    Ok(router.into())
}

