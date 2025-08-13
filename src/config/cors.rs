use actix_cors::Cors;
use actix_web::http::header;

pub fn options() -> Cors {
    Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials()
}
