use serde::Serialize;
use serde_derive::Deserialize;
use std::convert::Infallible;
use warp::{hyper::StatusCode, Rejection, Reply};

use crate::db::errors::DBError;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ErrorResponse {
    pub message: String,
}

pub async fn error_handler(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (status, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Resource not found")
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "Method not allowed")
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        eprintln!("BodyDeserializeError error: {:?}", e);
        (StatusCode::BAD_REQUEST, "Invalid request body")
    } else if let Some(e) = err.find::<warp::reject::InvalidQuery>() {
        eprintln!("InvalidQuery error: {:?}", e);
        (StatusCode::BAD_REQUEST, "Invalid query parameters")
    } else if let Some(db_error) = err.find::<DBError>() {
        match db_error {
            DBError::DBPoolConnection(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database connection error",
            ),
            DBError::DBQuery(_) => (StatusCode::BAD_REQUEST, "Database query failed"),
            DBError::ReadFile(_) => (StatusCode::INTERNAL_SERVER_ERROR, "File read error"),
            DBError::DBTimeout(_) => (StatusCode::REQUEST_TIMEOUT, "Database operation timed out"),
        }
    } else {
        eprintln!("Unhandled error: {:?}", err); // Ensure all unexpected errors are logged.
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    };

    let json = warp::reply::json(&ErrorResponse {
        message: message.to_string(),
    });

    Ok(warp::reply::with_status(json, status))
}
