use actix_web::{web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::models::product::Product;
use crate::services::product_service;

pub async fn get_products(pool: web::Data<MySqlPool>) -> impl Responder {
    let pool = pool.get_ref();
    match product_service::get_products(pool).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_product_by_id(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    let pool = pool.get_ref();
    match product_service::get_product_by_id(pool, id.into_inner()).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_product(pool: web::Data<MySqlPool>, product: web::Json<Product>) -> impl Responder {
    let pool = pool.get_ref();
    match product_service::create_product(pool, product.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_product(pool: web::Data<MySqlPool>, id: web::Path<i32>, product: web::Json<Product>) -> impl Responder {
    let pool = pool.get_ref();
    match product_service::update_product(pool, id.into_inner(), product.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_product(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    let pool = pool.get_ref();
    match product_service::delete_product(pool, id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
// Path: src/controllers/product_controller.rs