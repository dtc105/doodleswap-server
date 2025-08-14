use crate::controllers::auth::*;
use crate::middleware::authentication::AuthenticationMiddleware;

use actix_web::web::{ServiceConfig, get, patch, post, scope};

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .route("/login", post().to(login))
            .route("/register", post().to(register))
            .service(
                scope("")
                    .wrap(AuthenticationMiddleware)
                    .route("/token", get().to(read_token))
                    .route("/password", patch().to(change_password))
                    .route("/username", patch().to(change_username))
            )
    );
}
