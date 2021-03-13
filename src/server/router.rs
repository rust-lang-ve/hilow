use actix_web::web::{get, post, resource, ServiceConfig};

use crate::server::handlers::{health_check, signup};

/// Registers HTTP services to the current server instance
pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(resource("/signup").route(post().to(signup)));
    cfg.service(resource("/health_check").route(get().to(health_check)));
}
