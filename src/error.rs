use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use std::fmt;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct ServerError {
    pub e_code: u16,
    pub message: String,
}

impl ServerError {
    pub fn new(e_code: u16, message: String) -> ServerError {
        ServerError{
            e_code,
            message
        }
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message.to_string())
    }
}

impl From<diesel::result::Error> for ServerError {
    fn from(err: diesel::result::Error) -> ServerError {
        match err {
            diesel::result::Error::NotFound => ServerError::new(404, String::from("Record not found")),
            err => ServerError::new(500, String::from(format!("Diesel Error: {}", err)))
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.e_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let e_message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": e_message }))
    }
}
