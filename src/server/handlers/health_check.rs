use actix_web::{web, HttpResponse};
use serde::Serialize;

use crate::state::State;

/// Service Health Check
#[derive(Serialize)]
struct HealtCheckResponse {
    db_status: bool,
}

pub async fn health_check(state: web::Data<State>) -> HttpResponse {
    match sqlx::query("SELECT 1")
        .fetch_one(state.db_pool.as_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().json(HealtCheckResponse { db_status: true }),
        Err(_) => HttpResponse::InternalServerError().json(HealtCheckResponse { db_status: false }),
    }
}
