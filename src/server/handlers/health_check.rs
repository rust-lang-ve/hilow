use actix_web::{web, HttpResponse};
use serde::Serialize;

use crate::state::State;

pub async fn health_check(state: web::Data<State>) -> HttpResponse {
    let db_status = sqlx::query("SELECT 1")
        .fetch_one(state.db_pool.as_ref())
        .await
        .is_ok();
    let service_status = ServiceStatus::new(db_status);

    service_status.into()
}

/// Service Health Check
#[derive(Serialize)]
struct ServiceStatus {
    db_status: bool,
}

impl ServiceStatus {
    pub fn new(db_status: bool) -> Self {
        Self { db_status }
    }
}

impl From<ServiceStatus> for HttpResponse {
    fn from(status: ServiceStatus) -> HttpResponse {
        match status.db_status {
            true => HttpResponse::Ok().json(status),
            false => HttpResponse::InternalServerError().json(status),
        }
    }
}
