use crate::env::Env;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use log::info;
use crate::application::ports::article_service::ArticleService;
use crate::application::ports::category_service::CategoryService;
use crate::application::services::article_service::ArticleServiceImpl;
use crate::application::services::category_service::CategoryServiceImpl;
use crate::domain::repositories::article_repository::ArticleRepository;
use crate::domain::repositories::category_repository::CategoryRepository;
use crate::infrastructure::http::article_handler::{create_article, get_article, list_articles};
use crate::infrastructure::http::category_handler::{create_category, get_category, list_categories};
use crate::infrastructure::repositories::in_memory_article_repository::InMemoryArticleRepository;
use crate::infrastructure::repositories::in_memory_category_repository::InMemoryCategoryRepository;

mod application;
mod domain;
mod env;
mod infrastructure;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::from_env();
    tracing_subscriber::fmt::init();


    let article_repository: Box<dyn ArticleRepository + Send + Sync> =
        Box::new(InMemoryArticleRepository::new());
    let category_repository: Box<dyn CategoryRepository + Send + Sync> =
        Box::new(InMemoryCategoryRepository::new());

    let article_service: Data<Box<dyn ArticleService + Send + Sync>> = web::Data::new(Box::new(ArticleServiceImpl::new(article_repository)));
    let category_service: Data<Box<dyn CategoryService + Send + Sync>> = web::Data::new(Box::new(CategoryServiceImpl::new(category_repository)));

    info!("Starting server at {}:{}", env.host, env.port);
    HttpServer::new(move || {
        App::new()
            .app_data(article_service.clone())
            .app_data(category_service.clone())
            .service(list_articles)
            .service(create_article)
            .service(get_article)
            .service(list_categories)
            .service(get_category)
            .service(create_category)
    })
    .bind((env.host, env.port))?
    .run()
    .await?;

    Ok(())
}
