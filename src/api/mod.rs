use actix_web::{HttpResponse, Responder};

pub mod user_handle;

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}
