use crate::controllers::character::*;
use crate::middleware::authentication::AuthenticationMiddleware;

use actix_web::web::{ServiceConfig, get, post, scope};

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/characters")
            .route("/{character_id}", get().to(get_character))
            .service(
                scope("")
                    .wrap(AuthenticationMiddleware)
                    .route("/", post().to(create_character))
            )
    );
}