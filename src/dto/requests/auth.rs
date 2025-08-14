use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginCredentials {
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegistrationCredentials {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EmailChange {
    #[serde(rename = "newEmail")]
    #[validate(email)]
    pub new_email: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UsernameChange {
    #[serde(rename = "newUsername")]
    #[validate(length(min = 3, max = 32))]
    pub new_username: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PasswordChange {
    #[serde(rename = "newPassword")]
    #[validate(length(min = 6, max = 128))]
    pub new_password: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}