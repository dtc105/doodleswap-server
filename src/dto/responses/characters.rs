use crate::models::characters as models;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Character {
	pub id: i32,
	pub first_name: String,
	pub last_name: Option<String>,
	pub pronouns: Option<String>,
	pub quote: Option<String>,
}

impl From<models::Character> for Character {
	fn from(character: models::Character) -> Self {
		Self {
			id: character.id,
			first_name: character.first_name,
			last_name: character.last_name,
			pronouns: character.pronouns,
			quote: character.quote
		}
	}
}

#[derive(Debug, Serialize)]
pub struct Characters(Vec<Character>);

impl From<Vec<models::Character>> for Characters {
	fn from(vec: Vec<models::Character>) -> Self {
		Self(
			vec.into_iter()
			 	.map(Character::from)
				.collect()
		)
	}
}

#[derive(Debug, Serialize)]
pub struct CreatedCharacter {
	pub id: i32,
	pub image_path: String,
	pub image_mime_type: String
}