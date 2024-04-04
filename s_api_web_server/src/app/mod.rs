use actix_web::web;

pub mod hello;

pub fn register_urls(cfg: &mut web::ServiceConfig) {
    hello::register_urls(cfg);
}
