use crate::libs::services::hello::service::greeting;
use actix_web::{get, HttpResponse, Responder};

#[get("")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body(greeting("world"))
}
