use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub id: i32,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct RegistrationResponse {
    pub id: i32,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub id: i32,
    pub username: String,
    pub role: String,
}