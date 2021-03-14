use actix_web::web::{post, resource, ServiceConfig};

use crate::handlers::signup;

/// Registers HTTP services to the current server instance
pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(resource("/signup").route(post().to(signup)));
}
