use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub id: i64,
    pub username: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct RegistrationResponse {
    pub id: i64,
    pub username: String,
    pub role: String,
}
