use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize}; // (d1)

#[derive(Deserialize)] // (d2)
struct HelloQuery {
    name: String,
    age: u32,
}

#[derive(Serialize)] // (d3)
struct HelloResponse {
    greet: String,
}

#[actix_web::get("/")]
async fn hello(
    query: web::Query<HelloQuery>,
) -> HttpResponse {
    let query = query.into_inner(); // (d4)
    let message = format!(
        "Hello, my name is {}! I am {} years old!",
        query.name, query.age
    );
    let h = HelloResponse { greet: message }; // (d5)

    HttpResponse::Ok().json(h) // (d6)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
