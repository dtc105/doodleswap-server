use super::HttpErrorBody;

use actix_web::HttpResponse;

pub fn failed_creation() -> HttpResponse {
    HttpResponse::BadRequest().json(HttpErrorBody {
        error: "Bad Request".to_string(),
        message: "Failed to create character".to_string(),
    })
}

pub fn too_many_characters() -> HttpResponse {
    HttpResponse::Forbidden().json(HttpErrorBody {
        error: "Forbidden".to_string(),
        message: "Not allowed to create more characters.".to_string(),
    })
}

pub fn unsupported_file() -> HttpResponse {
    HttpResponse::UnsupportedMediaType().json(HttpErrorBody {
        error: "Unsupported Media Type".to_string(),
        message: "Only image/png and image/jpeg allowed.".to_string(),
    })
}

pub fn file_too_large() -> HttpResponse {
    HttpResponse::PayloadTooLarge().json(HttpErrorBody {
        error: "Payload Too Large".to_string(),
        message: "File size must be 5MiB or less.".to_string(),
    })
}
