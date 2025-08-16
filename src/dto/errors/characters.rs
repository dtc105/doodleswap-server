use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct CharacterError {
    error: String,
    message: String,
}

pub fn failed_creation() -> CharacterError {
    CharacterError {
        error: "Bad Request".to_string(),
        message: "Failed to create character".to_string(),
    }
}

pub fn too_many_characters() -> CharacterError {
    CharacterError {
        error: "Forbidden".to_string(),
        message: "Not allowed to create more characters.".to_string()
    }
}
