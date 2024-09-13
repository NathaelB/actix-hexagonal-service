use crate::application::article_service::ArticleService;
use crate::domain::repository::{ArticleRepository, CategoryRepository};
use crate::env::Env;
use crate::infrastructure::in_memory_repository::{InMemoryArticleRepository, InMemoryCategoryRepository};
use actix_web::{web, App, HttpServer};
use log::info;
use crate::application::category_service::CategoryService;
use crate::http::article_handler::{create_article, get_article, list_articles};

mod application;
mod domain;
mod env;
mod infrastructure;
mod http;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::from_env();
    tracing_subscriber::fmt::init();

    let article_repository: Box<dyn ArticleRepository + Send + Sync> =
        Box::new(InMemoryArticleRepository::new());
    let category_repository: Box<dyn CategoryRepository + Send + Sync> =
        Box::new(InMemoryCategoryRepository::new());

    let article_service = web::Data::new(ArticleService::new(article_repository));
    let category_service = web::Data::new(CategoryService::new(category_repository));

    info!("Starting server at {}:{}", env.host, env.port);
    HttpServer::new(move || {
        App::new()
            .app_data(article_service.clone())
            .app_data(category_service.clone())
            .service(list_articles)
            .service(create_article)
            .service(get_article)
    })
    .bind((env.host, env.port))?
    .run()
    .await?;

    Ok(())
}
