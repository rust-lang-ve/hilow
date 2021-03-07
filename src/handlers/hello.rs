use actix_web::Responder;

pub async fn hello() -> impl Responder {
    String::from("Hello!")
}
