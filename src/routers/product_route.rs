use axum::{routing::get, Router};

pub fn product_routes() -> Router {
    Router::new().route("/product", get(|| async { "Hello, This is Product" }))
}
