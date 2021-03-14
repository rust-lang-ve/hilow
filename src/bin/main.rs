use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::Context;
use hilow::router::router;
use hilow::state::State;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env").expect("Unable to find \".env\" in the CWD");
    }

    env_logger::init();

    log::info!("Serving on http://0.0.0.0:7878");

    let state = State::new().context("Failed to initialze \"State\"!")?;
    let state = Data::new(state);

    HttpServer::new(move || {
        // `Data<State>` is being cloned to avoid creating
        // one `State` instance per worker.
        // Otherwise, `Data<State>` will be intialized as many
        // times as the total count of workers configured
        App::new().app_data(state.clone()).configure(router)
    })
    .bind("0.0.0.0:7878")?
    .run()
    .await?;
    Ok(())
}
