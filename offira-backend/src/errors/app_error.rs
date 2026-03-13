use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response}, // Response -> actual response type
};
use serde_json::json; // Convert rust values to json

pub struct AppError {
    pub status: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn internal(msg: &str) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: msg.to_string(),
        }
    }

    pub fn not_found(msg: &str) -> Self {
    Self {
        status: StatusCode::NOT_FOUND,
        message: msg.to_string(),
    }
}
}

// Router returns Result<SuccessType, ErrorType>
impl IntoResponse for AppError {
    // IntoResponse = AppError -> HTTP Response
    fn into_response(self) -> Response {
        let body = Json(json!({
            "success": false,
            "error": self.message
        }));

        (self.status, body).into_response()
    }
}
