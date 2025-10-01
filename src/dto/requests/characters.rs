use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCharacterData {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub pronouns: Option<String>,
    pub quote: Option<String>,
}
