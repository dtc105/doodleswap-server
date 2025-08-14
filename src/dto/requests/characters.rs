use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCharacter {
	pub first_name: String,
	pub last_name: Option<String>,
	pub pronouns: Option<String>,
	pub quote: Option<String>
}