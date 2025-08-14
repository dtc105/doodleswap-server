use crate::controllers::auth::*;

use actix_web::web::{ServiceConfig, post, scope};

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .route("/login", post().to(login))
            .route("/register", post().to(register)),
    );
}
