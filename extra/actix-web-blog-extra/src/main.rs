mod error;
mod repository;
mod schema;
mod types;

use actix_web::{
    middleware::Logger, middleware::NormalizePath,
    web, App, HttpResponse, HttpServer,
};
use error::ApiError;
use repository::{
    NewPost, PostChangeset, PostPublishRequest,
    Repository,
};

#[actix_web::post("/posts")]
async fn create_post(
    repo: web::Data<Repository>,
    new_post: web::Json<NewPost>,
) -> Result<HttpResponse, ApiError> {
    let new_post = new_post.into_inner();
    let post = repo.create_post(new_post).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[actix_web::get("/posts")]
async fn list_posts(
    repo: web::Data<Repository>,
) -> Result<HttpResponse, ApiError> {
    let res = repo.list_posts().await?;
    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::get("/posts/{id}")]
async fn get_post(
    repo: web::Data<Repository>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let res = repo.get_post(id).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[actix_web::patch("/posts/{id}")]
async fn update_post(
    repo: web::Data<Repository>,
    path: web::Path<i32>,
    changeset: web::Json<PostChangeset>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let changset = changeset.into_inner();
    let post = repo.update_post(id, changset).await?;
    Ok(HttpResponse::Ok().json(post))
}

#[actix_web::put("/posts/{id}")]
async fn publish_post(
    repo: web::Data<Repository>,
    path: web::Path<i32>,
    req_publish: web::Json<PostPublishRequest>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    let req = req_publish.into_inner();
    repo.publish_post(id, req.published).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[actix_web::delete("/posts/{id}")]
async fn delete_post(
    repo: web::Data<Repository>,
    path: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();
    repo.delete_post(id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init(); // 追加

    let database_url =
        std::env::var("DATABASE_URL").unwrap();
    let repo = web::Data::new(Repository::new(
        &database_url,
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(repo.clone())
            .wrap(Logger::default()) // 追加
            .wrap(NormalizePath::trim()) // 追加
            .service(create_post)
            .service(list_posts)
            .service(get_post)
            .service(update_post)
            .service(publish_post)
            .service(delete_post)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
