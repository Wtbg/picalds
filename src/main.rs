use actix_web::{get, post, web::Json, App, HttpResponse, HttpServer, Responder};
pub mod handlers;
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8888")?
        .run()
        .await?;
    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    Json(handlers::ApiResponse::Success("Hello, world!".to_string()))
}

#[post("/upload")]
async fn upload() -> impl Responder {
    Json(handlers::ApiResponse::Success("Hello, world!".to_string()))
}
