use axum::{
    routing::{get, post},
    Router,
};

use crate::controllers::user_controller::{add_user, delete_user, get_user, update_user};

pub fn user_routes() -> Router {
    Router::new()
        .route("/getUser/:id", get(get_user))
        .route("/addUser", post(add_user))
        .route("/udateUser/:id", get(update_user))
        .route("/deleteUser/:id", get(delete_user))
}
