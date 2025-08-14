use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AuthenticationError {
    error: String,
    message: String,
}

pub fn unauthorized(message: &str) -> AuthenticationError {
    AuthenticationError {
        error: "Unauthorized".to_string(),
        message: message.to_string(),
    }
}

pub fn username_taken() -> AuthenticationError {
    AuthenticationError {
        error: "Conflict".to_string(),
        message: "Username taken.".to_string(),
    }
}

pub fn email_taken() -> AuthenticationError {
    AuthenticationError {
        error: "Conflict".to_string(),
        message: "Email taken".to_string(),
    }
}
