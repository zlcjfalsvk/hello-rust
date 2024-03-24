use crate::app::hello::views::hello;
use actix_web::web;

mod views;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/hello").service(hello));
}
