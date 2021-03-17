use actix_web::{http::StatusCode, test, web::Data, App};

use hilow::state::State;

#[actix_rt::test]
async fn health_check_works() {
    std::env::set_var("DATABASE_URL", "postgres://hilow:hilow@127.0.0.1/hilow");
    let state = State::new().expect("Failed to initialze \"State\"!");
    let state = Data::new(state);

    let mut app =
        test::init_service(App::new().app_data(state).configure(hilow::server::router)).await;

    let req = test::TestRequest::get().uri("/health_check").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[actix_rt::test]
async fn health_check_works_failure() {
    // Set wrong password in order to produce health_check error
    std::env::set_var("DATABASE_URL", "postgres://hilow:h@127.0.0.1/hilow");

    let state = State::new().expect("Failed to initialze \"State\"!");
    let state = Data::new(state);

    let mut app =
        test::init_service(App::new().app_data(state).configure(hilow::server::router)).await;

    let req = test::TestRequest::get().uri("/health_check").to_request();

    let resp = test::call_service(&mut app, req).await;

    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
}
