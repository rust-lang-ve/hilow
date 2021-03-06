#[macro_use]
extern crate log;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env.development")
            .expect("Unable to find \".env.development\" in the CWD");
    }

    env_logger::init();

    info!("Serving on http://0.0.0.0:7878");
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:7878")?
        .run()
        .await
}
