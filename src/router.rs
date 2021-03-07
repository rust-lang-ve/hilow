use actix_web::web::{get, resource, ServiceConfig};

use crate::handlers::hello;

/// Registers HTTP services to the current server instance
pub fn router(cfg: &mut ServiceConfig) {
    cfg.service(resource("/hello").route(get().to(hello)));
}
