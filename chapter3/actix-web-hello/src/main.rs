use actix_web::{ // (d1)
    http::header::ContentType, App, HttpResponse,
    HttpServer,
};

#[actix_web::get("/")] // (d2)
async fn hello() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(ContentType::json())
        .body(r#"{"greet":"Hello, world!"}"#)
}

#[actix_web::main] // (d3)
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
