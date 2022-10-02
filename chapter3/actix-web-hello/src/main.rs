use actix_web::{
    http::header::ContentType, App, HttpResponse,
    HttpServer,
};

#[actix_web::get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(ContentType::json())
        .body(r#"{"greet":"Hello, world!"}"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
