use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct HelloQuery {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct HelloResponse {
    greet: String,
}

#[actix_web::get("/")]
async fn hello(
    query: web::Query<HelloQuery>,
) -> HttpResponse {
    let query = query.into_inner();
    let message = format!(
        "Hello, my name is {}! I am {} years old!",
        query.name, query.age
    );
    let h = HelloResponse { greet: message };

    HttpResponse::Ok().json(h)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
