use actix_multipart::Multipart;
use actix_web::{
    get, post, web, web::Json, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use futures::{StreamExt, TryStreamExt};
use nanoid::nanoid;
use sqlx::mysql::MySqlPool;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;
pub mod handlers;
pub mod models;
const TEMP_DIR: &str = "./image";
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let pool = MySqlPool::connect("mysql://root:123456@localhost:3306/test")
    //     .await
    //     .unwrap();
    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(pool.clone()))
            .service(index)
            .service(upload)
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await?;
    Ok(())
}

#[get("/")]
async fn index() -> impl Responder {
    Json(handlers::ApiResponse::Success("Hello, world!".to_string()))
}
fn save_file_create(folder: String, name: String) {
    let filepath = std::path::Path::new(TEMP_DIR).join(folder).join(name);
    std::fs::File::create(&filepath).unwrap();
}

fn save_file_add(folder: String, name: String, file: web::Bytes) -> Result<(), std::io::Error> {
    let filepath = std::path::Path::new(TEMP_DIR).join(folder).join(name);
    let mut f = std::fs::OpenOptions::new()
        .append(true)
        .open(&filepath)
        .unwrap();
    f.write_all(&file)
}
#[post("/upload/{user_id}")]
async fn upload(
    user_id: actix_web::web::Path<String>,
    mut payload: actix_multipart::Multipart,
    // pool: web::Data<MySqlPool>,
    request: HttpRequest,
) -> impl Responder {
    let file_type = request.headers().get("picture-type").unwrap();
    let file_type = match file_type.to_str() {
        Ok(s) => match s.to_string() {
            s if s == "png" => "png",
            s if s == "jpg" => "jpg",
            s if s == "jpeg" => "jpeg",
            _ => panic!("file type error"),
        },
        Err(_) => panic!("file type error"),
    };
    let pid = nanoid!(6);
    let file_name = format!("{}.{}", pid, file_type);
    save_file_create(user_id.to_string(), file_name.clone());
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            save_file_add(user_id.to_string(), file_name.clone(), data).unwrap();
        }
    }
    Json(handlers::ApiResponse::Success(models::PictureFile {
        pid: pid,
        url: format!("http://localhost:8888/image/{}/{}", user_id.to_string(), file_name),
        owner: user_id.to_string(),
    }))
}
