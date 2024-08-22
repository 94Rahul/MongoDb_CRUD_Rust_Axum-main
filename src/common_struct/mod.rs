use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
    pub errors: Option<T>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}
pub async fn handle_db_error<T: std::fmt::Debug>(error: T) -> (StatusCode, Json<Value>) {
    println!("Database Error: {:?}", error);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!(ApiResponse {
            status: "error".to_string(),
            code: 500,
            message: "Internal server error".to_string(),
            data: None,
            errors: Some(format!("{:?}", error)),
        })),
    )
}

pub async fn handle_invalid_id_error<T>(params: String) -> (StatusCode, Json<Value>) {
    println!("Invalid ID format: {}", params);
    (
        StatusCode::BAD_REQUEST,
        Json(json!(ApiResponse {
            status: "error".to_string(),
            code: 400,
            message: "Invalid ID format".to_string(),
            data: None,
            errors: Some(format!("Invalid ID format: {}", params)),
        })),
    )
}
