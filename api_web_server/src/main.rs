mod app;
mod libs;

use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().message_body(".") }),
            )
            .service(web::scope("/api").configure(app::register_urls))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
