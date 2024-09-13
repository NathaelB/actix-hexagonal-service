use actix_web::{get, HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use crate::application::category_service::CategoryService;

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
}

#[post("/categories")]
async fn create_category(
    service: web::Data<CategoryService>,
    request: web::Json<CreateCategoryRequest>
) -> impl Responder {
    let category = service.create_category(request.name.clone()).await;

    match category {
        Some(c) => HttpResponse::Created().json(c),
        None => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/categories")]
async fn list_categories(
    service: web::Data<CategoryService>,
) -> impl Responder {
    let categories = service.list_categories().await;
    HttpResponse::Ok().json(categories)
}

