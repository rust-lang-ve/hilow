use std::net::TcpListener;

use anyhow::Result;

use hilow;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app().expect("Unable to spawn server");
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to  execute request.");

    assert!(response.status().is_success());
}

#[actix_rt::test]
async fn health_check_works_failure() {
    std::env::set_var("DATABASE_URL", "postgres://hilow:h@127.0.0.1/hilow");
    let address = spawn_app().expect("Unable to spawn server");

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to  execute request.");

    assert!(response.status().is_server_error());
}

/// Spawn a hilow server and return its address to the caller.
fn spawn_app() -> Result<String> {
    // Port 0 translates to a random port.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = hilow::server::run(listener).unwrap();
    let _ = tokio::spawn(server);

    Ok(format!("http://127.0.0.1:{}", port))
}
