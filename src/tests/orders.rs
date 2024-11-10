#[cfg(test)]
mod tests {
    use axum::{body::{Body, to_bytes}, http::{Request, StatusCode}};
    use axum::Router;
    use serde_json::json;
    use std::sync::Arc;
    use tower::ServiceExt; // for `oneshot`
    use crate::routes::orders;
    use crate::state::app_state::AppState;


    fn setup_router() -> Router {
        let state = Arc::new(AppState::default());
        orders::routes()
            .with_state(state)

    }

    #[tokio::test]
    async fn test_add_order() {
        let app = setup_router();

        let payload = json!({
            "table_number": "1",
            "items": [{ "name": "Pizza" }]
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/orders")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_query_table() {
        let app = setup_router();

        // Add an order first
        let payload = json!({
            "table_number": "1",
            "items": [{ "name": "Pizza" }]
        });
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/orders")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Query the table
        let response = app
            .oneshot(Request::builder().uri("/orders/1").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("Pizza"));
    }

    #[tokio::test]
    async fn test_remove_item() {
        let app = setup_router();

        // Add an order first
        let payload = json!({
            "table_number": "1",
            "items": [{ "name": "Pizza" }]
        });
        app.clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/orders")
                    .header("content-type", "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Remove the item
        let response = app
            .oneshot(Request::builder().method("DELETE").uri("/orders/1/Pizza").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("Item 'Pizza' removed successfully"));
    }
}
