use actix_web::{test, web, http, App};
use actix_web::http::header;
use actix_rt;
use ::todolist_lib::api;

#[actix_rt::test]
async fn test_health_check_get() {
    let mut app = test::init_service(
        App::new()
            .route("/health_check", web::get().to(api::health_check))
    ).await;
    let req = test::TestRequest::default().insert_header((
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/text"),
    )).uri("/health_check").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), http::StatusCode::OK);
}

