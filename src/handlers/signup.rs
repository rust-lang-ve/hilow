use actix_web::web::{Data, Json};
use actix_web::Responder;

use crate::domain::services::dto::user::RegisterUserDto;
use crate::state::State;

pub async fn signup(state: Data<State>, payload: Json<RegisterUserDto>) -> impl Responder {
    match state
        .services
        .user_service
        .register(payload.into_inner())
        .await
    {
        Ok(_) => "Successful Request".to_string(),
        Err(e) => format!("Request failed! {}", e.to_string()),
    }
}
