mod product_route;
mod user_route;
use axum::Router;
use product_route::product_routes;
use user_route::user_routes;
pub async fn router() -> Router {
    let router = Router::new().merge(user_routes()).merge(product_routes());
    router
}
