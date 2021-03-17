use hilow::server::run;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    if cfg!(debug_assertions) {
        dotenv::from_filename(".env").expect("Unable to find \".env\" in the CWD");
    }

    env_logger::init();
    run("0.0.0.0:7878").await?;
    Ok(())
}
