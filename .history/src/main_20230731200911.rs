use actix_web::{
    get, post,
    web::{self, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
// use sqlx::mysql::MySqlPoolOptions;

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
    HttpResponse::Ok().body("Hello world!")
}

#[post("/upload")]
async fn upload() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("image/{image_id}")]
async fn get_image() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}