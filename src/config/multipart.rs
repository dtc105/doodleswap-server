use actix_multipart::form::MultipartFormConfig;
use actix_web::error;

pub fn init() -> MultipartFormConfig {
    MultipartFormConfig::default()
        .total_limit(5 * 1024 * 1024)
        .error_handler(|e, _| error::ErrorPayloadTooLarge(e.to_string()))
}
