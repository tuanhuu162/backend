use actix_web::{App, http, test, web};
use actix_web::http::{header, Method};
use actix_service::Service;
use serde_json::json;
use pretty_assertions::assert_eq;
use std::str;

use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, ConnectionManager};




pub fn init_app(path: &str, mut func: impl FnMut() + Copy) -> Service{
    let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
    let pool = Pool::builder().build(manager).expect("Failed to create pool");

    test::init_service(
        App::new()
            .data(pool.clone())
            .route(path, web::post().to(func))
    ).await
}


pub fn test_request(app:&mut Service, headers: header::IntoHeaderPair, uri: &str, method: Method, body: serde_json::Value, response: String) {
    let req1 = test::TestRequest::default().insert_header(headers)
    .uri(uri)
    .method(method)
    .set_json(body)
    .to_request();
    let mut resp1 = test::call_service(app, req1).await;
    let body1 = resp1.take_body();
    let response_body1 = match body1.as_ref() {
        Some(actix_web::body::Body::Bytes(bytes1)) => str::from_utf8(bytes1).unwrap(),
        _ => panic!("Response error"),
    };
    println!("{}", response_body1);
    assert_eq!(response_body1, response);
}