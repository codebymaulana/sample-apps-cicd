use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust!")
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
