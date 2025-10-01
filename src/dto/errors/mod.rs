use actix_web::HttpResponse;
use serde::Serialize;

pub mod auth;
pub mod characters;

#[derive(Debug, Serialize)]
pub struct HttpErrorBody {
    error: String,
    message: String,
}
