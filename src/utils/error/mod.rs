use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use log::error;
use serde_json::json;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct CustomError {
    pub status_code: StatusCode,
    pub message: String,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status_code, self.message)
    }
}

impl Error for CustomError {}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        error!("Error: {}", self.message);
        let body = Json(json!({ "error": self.message }));
        (self.status_code, body).into_response()
    }
}
