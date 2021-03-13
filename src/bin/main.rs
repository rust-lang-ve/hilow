#[macro_use]
extern crate log;

use actix_web::{web, App, HttpServer};

use hilow::router::router;
use hilow::state::init_app_state;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env.development")
            .expect("Unable to find \".env.development\" in the CWD");
    }

    env_logger::init();

    let app_state = web::Data::new(init_app_state().await?);

    info!("Serving on http://0.0.0.0:7878");
    HttpServer::new(move || App::new().app_data(app_state.clone()).configure(router))
        .bind("0.0.0.0:7878")?
        .run()
        .await?;
    Ok(())
}
