use super::HttpErrorBody;

use actix_web::HttpResponse;

pub fn incorrect_credentials() -> HttpResponse {
    HttpResponse::Unauthorized().json(HttpErrorBody {
        error: "Unauthorized".to_string(),
        message: "Username or password incorrect.".to_string(),
    })
}

pub fn username_taken() -> HttpResponse {
    HttpResponse::Conflict().json(HttpErrorBody {
        error: "Conflict".to_string(),
        message: "Username taken.".to_string(),
    })
}

pub fn email_taken() -> HttpResponse {
    HttpResponse::Conflict().json(HttpErrorBody {
        error: "Conflict".to_string(),
        message: "Email taken".to_string(),
    })
}
