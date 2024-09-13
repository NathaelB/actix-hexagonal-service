use crate::application::article_service::ArticleService;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub content: String,
}

#[post("/articles")]
pub async fn create_article(
    service: web::Data<ArticleService>,
    request: web::Json<CreateArticleRequest>,
) -> impl Responder {
    let article = service
        .create_article(request.title.clone(), request.content.clone())
        .await;

    match article {
        Some(a) => HttpResponse::Created().json(a),
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/articles/{article_id}")]
pub async fn get_article(
    service: web::Data<ArticleService>,
    article_id: web::Path<u32>,
) -> impl Responder {
    match service.get_article(article_id.into_inner()).await {
        Some(article) => HttpResponse::Ok().json(article),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/articles")]
pub async fn list_articles(service: web::Data<ArticleService>) -> impl Responder {
    let articles = service.list_articles().await;
    HttpResponse::Ok().json(articles)
}
