use actix_web::{get, App, HttpResponse, HttpServer, Responder};

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
    HttpResponse::Ok().body("welcome to picalds")
}
