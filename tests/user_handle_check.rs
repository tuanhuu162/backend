use actix_web::{test, web, App};
use actix_web::http::{header, Method};
use actix_rt;
use ::todolist_lib::{api, vars};
use serde_json::json;
use std::str;

use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager};

use pretty_assertions::assert_eq;

#[actix_rt::test]
async fn test_greet_get() {
    let mut app = test::init_service(
        App::new()
            .route("/", web::get().to(api::user_handle::greet))
    ).await;
    let req = test::TestRequest::default().insert_header((
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/text"),
    ))
    .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
    let body = resp.take_body();
    let response_body = match body.as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => str::from_utf8(bytes).unwrap(),
        _ => panic!("Response error"),
    };
    println!("{:?}", response_body);
    assert_eq!(response_body, String::from(r##"{"messege":"Hello World"}"##));

    println!("Finished test 1");
    let req1 = test::TestRequest::default().insert_header((
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/text"),
    )).uri("/?name=Tuan").to_request();
    let mut resp1 = test::call_service(&mut app, req1).await;
    let body1 = resp1.take_body();
    let response_body1 = match body1.as_ref() {
        Some(actix_web::body::Body::Bytes(bytes1)) => str::from_utf8(bytes1).unwrap(),
        _ => panic!("Response error"),
    };
    println!("{:?}", response_body1);
    assert_eq!(response_body1, String::from(r##"{"messege":"Hello Tuan"}"##))
}

#[actix_rt::test]
async fn test_register_post() {
    
    let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
    let pool = Pool::builder().build(manager).expect("Failed to create pool");

    let mut app = test::init_service(
        App::new()
            .data(pool.clone())
            .route("/register", web::post().to(api::user_handle::register))
    ).await;

    println!("Finished test");
    let req = test::TestRequest::default().insert_header((
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    ))
    .uri("/register")
    .method(Method::POST)
    .set_json(&json!({
        "email": "tuanhuu162@gmail.com",
        "name": "tuan",
        "password": "tuanhuu"
    }))
    .to_request();
    let mut resp = test::call_service(&mut app, req).await;
    let body = resp.take_body();
    let response_body = match body.as_ref() {
        Some(actix_web::body::Body::Bytes(bytes)) => str::from_utf8(bytes).unwrap(),
        _ => panic!("Response error"),
    };
    println!("{}", response_body);
    assert_eq!(response_body, String::from(r##"{"messege":"Successful register user!!!!!!"}"##));

    println!("Finished test");
    let req1 = test::TestRequest::default().insert_header((
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    ))
    .uri("/register")
    .method(Method::POST)
    .set_json(&json!({
        "email": "tuanhuu162@gmail.com",
        "name": "tuan"
    }))
    .to_request();
    let mut resp1 = test::call_service(&mut app, req1).await;
    let body1 = resp1.take_body();
    let response_body1 = match body1.as_ref() {
        Some(actix_web::body::Body::Bytes(bytes1)) => str::from_utf8(bytes1).unwrap(),
        _ => panic!("Response error"),
    };
    println!("{}", response_body1);
    assert_eq!(response_body1, String::from(r##"{"messege":"Lack of parameter!!!!!!!!!!!"}"##));
}

