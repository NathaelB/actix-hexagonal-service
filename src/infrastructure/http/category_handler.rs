use actix_web::{get, HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use crate::application::ports::category_service::CategoryService;

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[post("/categories")]
pub async fn create_category(
    service: web::Data<dyn CategoryService>,
    request: web::Json<CreateCategoryRequest>
) -> impl Responder {
    let category = service.create(request.name.clone()).await;

    match category {
        Some(c) => HttpResponse::Created().json(c),
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/categories")]
pub async fn list_categories(
    service: web::Data<dyn CategoryService>,
) -> impl Responder {
    let categories = service.list().await;
    HttpResponse::Ok().json(categories)
}

#[get("/categories/{category_id}")]
pub async fn get_category(
    service: web::Data<dyn CategoryService>,
    category_id: web::Path<u32>,
) -> impl Responder {
    match service.get_by_id(category_id.into_inner()).await {
        Some(category) => HttpResponse::Ok().json(category),
        None => HttpResponse::NotFound().finish(),
    }
}

