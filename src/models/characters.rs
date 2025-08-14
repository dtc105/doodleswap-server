#[derive(Debug)]
pub struct Character {
	pub id: i32,
	pub user_id: i32,
	pub first_name: String,
	pub last_name: Option<String>,
	pub pronouns: Option<String>,
	pub quote: Option<String>,
	pub image_path: String,
	pub image_mime_type: String
}