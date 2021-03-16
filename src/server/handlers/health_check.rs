use actix_web::{web, HttpResponse};
use serde::Serialize;

use crate::state::State;

pub async fn health_check(state: web::Data<State>) -> HttpResponse {
    match sqlx::query("SELECT 1")
        .fetch_one(state.db_pool.as_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(ServiceStatus { db_status: true }),
        Err(_) => HttpResponse::InternalServerError().json(ServiceStatus { db_status: false }),
    }
}

/// Service Health Check
#[derive(Serialize)]
struct ServiceStatus {
    db_status: bool,
}
