use axum::Json;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde_json::json;

pub type Result<T> = core::result::Result<T,Error>;


#[derive(Debug)]
pub enum Error{
    LoginFail,

    // Auth Errors
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,

    TemplateError,


    TicketDeleteFailIdNotFound {id: String}
}

impl IntoResponse for Error{
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            Error::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            Error::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            Error::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            _ =>  (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR")
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
       .into_response()
    }
}