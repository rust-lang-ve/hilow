use std::net::TcpListener;

use actix_web::{dev::Server, web::Data, App, HttpServer};
use anyhow::{Context, Result};
use log::info;

use crate::server::router;
use crate::state::State;

pub fn run(listener: TcpListener) -> Result<Server> {
    let state = State::new().context("Failed to initialze \"State\"!")?;
    let state = Data::new(state);

    info!("Serving on http://{}", listener.local_addr()?);

    let server = HttpServer::new(move || {
        // `Data<State>` is being cloned to avoid creating
        // one `State` instance per worker.
        // Otherwise, `Data<State>` will be intialized as many
        // times as the total count of workers configured
        App::new().app_data(state.clone()).configure(router)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
