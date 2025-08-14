use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub role: String,
    pub pfp_path: Option<String>,
    pub pfp_mime_type: Option<String>,
    pub created_at: NaiveDateTime,
}