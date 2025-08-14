mod auth;

use actix_web::web::{ServiceConfig, scope};

pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(scope("").configure(auth::router));
}
