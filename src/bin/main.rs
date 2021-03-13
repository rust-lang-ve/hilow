use std::net::TcpListener;

use hilow::server::run;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env").expect("Unable to find \".env\" in the CWD");
    }

    env_logger::init();
    let listener =
        TcpListener::bind("0.0.0.0:7878").expect("Unable to bind the service in 0.0.0.0:7878");

    run(listener)?.await?;
    Ok(())
}
