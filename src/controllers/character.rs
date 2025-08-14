use crate::{
    config::database::AppState,
    dto::{
        errors::characters as err,
        requests::characters as req,
        responses::characters as res
    },
	middleware::authentication::Claims,
    models::characters as models,
};

use actix_multipart::Multipart;
use actix_web::{
    Error,
    HttpMessage,
    HttpRequest,
    HttpResponse,
    error,
    web::{Data, Path}
};
use futures_util::stream::StreamExt;
use serde_json;
use std::{
	io::Write,
	fs::{File, remove_file}
};
use uuid::Uuid;

pub async fn get_character(
	path: Path<i32>,
	state: Data<AppState>
) -> Result<HttpResponse, Error> {
	return Ok(HttpResponse::NotImplemented().body("Needs to send image to client."));
	let character_id = path.into_inner();

	let character: models::Character = sqlx::query_as!(
		models::Character,
		r#"
			SELECT *
			FROM doodleswap.character AS c
			WHERE c.id = $1;
		"#,
		character_id
	)
	.fetch_one(&state.pool)
	.await
	.map_err(|e| error::ErrorNotFound(e))?;

	Ok(HttpResponse::Ok().json(res::Character::from(character)))
}

pub async fn create_character(
	req: HttpRequest,
	mut payload: Multipart,
	state: Data<AppState>
) -> Result<HttpResponse, Error> {
	// Initialize the data
	let req_extensions = req.extensions();
	let claims = req_extensions
		.get::<Claims>()
		.ok_or_else(|| error::ErrorUnauthorized("No token."))?;

	let mut metadata: Option<req::CreateCharacter> = None;
	let mut path: Option<String> = None;

	// Go through every field
	while let Some(Ok(mut field)) = payload.next().await {
		let content_disposition = field.content_disposition().unwrap();
		let name = content_disposition.get_name().unwrap_or("");

		if name == "metadata" {
			// Initialize the metadata as a [u8]
			let mut data: Vec<u8> = Vec::new();

			// Read the metadata
			while let Some(Ok(chunk)) = field.next().await {
				data.extend_from_slice(&chunk);
			}

			// Convert the metadata into a req::CreateCharacter
			let json_str = String::from_utf8(data)
				.map_err(|e| error::ErrorBadRequest(e.to_string()))?;
			let json: req::CreateCharacter = serde_json::from_str(&json_str)
				.map_err(|e| error::ErrorBadRequest(e.to_string()))?;

			metadata = Some(json);
		} else if name == "file" {
			// Initialize the filenames and paths
            let filename = content_disposition.get_filename().unwrap_or("file.jpg");
            let extension = filename.split('.').last().unwrap_or("jpg");
            let saved_filename = format!("{}.{}", Uuid::new_v4(), extension);
            let saved_filepath = format!(
				"./uploads/users/{}/{}",
				claims.sub,
				saved_filename
			);

			let mut file = File::create(&saved_filepath)?;

			// Read the data and write it to a file
			while let Some(Ok(chunk)) = field.next().await {
				file.write_all(&chunk)?;
			}

			path = Some(saved_filepath);
		}
	}

	match (metadata, path) {
		(Some(metadata), Some(path)) => {
			// If everything went well, insert into database
			let inserted_id: i32 = sqlx::query_scalar!(
		        r#"
		            INSERT INTO doodleswap.character(
						first_name,
						last_name,
						pronouns,
						quote,
						image_path,
						image_mime_type
					)
		            VALUES (
						$1,
						$2,
						$3,
						$4,
						$5,
						'MIME TYPE'
					)
		            RETURNING id;
		        "#,
		        metadata.first_name,
				metadata.last_name,
				metadata.pronouns,
				metadata.quote,
				&path
		    )
		    .fetch_one(&state.pool)
		    .await
		    .map_err(|e| error::ErrorInternalServerError(e.to_string()))?;

			// Send the response
			Ok(HttpResponse::Created().json(res::CreatedCharacter {
				id: inserted_id,
				image_path: path,
				image_mime_type: String::from("MIME TYPE")
			}))
		},
		(None, Some(path)) => {
			// If the image was created but no metadata was provided
			// delete the image
			remove_file(path)?;
			Ok(HttpResponse::BadRequest().json(err::failed_creation()))
		}
		_ => Ok(HttpResponse::BadRequest().json(err::failed_creation()))
	}
}