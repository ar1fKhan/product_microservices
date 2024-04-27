use sqlx::MySqlPool;
use crate::models::product::Product;
use crate::repositories::product_repository::ProductRepository as product_repository;

pub async fn get_products(pool: &MySqlPool) -> Result<Vec<Product>, sqlx::Error> {
    product_repository::get_products(pool).await
}

pub async fn get_product_by_id(pool: &MySqlPool, id: i32) -> Result<Product, sqlx::Error> {
    product_repository::get_product_by_id(pool, id).await
}

pub async fn create_product(pool: &MySqlPool, product: Product) -> Result<u64, sqlx::Error> {
    product_repository::create_product(pool, product).await
}

pub async fn update_product(pool: &MySqlPool, id: i32, product: Product) -> Result<u64, sqlx::Error> {
    product_repository::update_product(pool, id, product).await
}

pub async fn delete_product(pool: &MySqlPool, id: i32) -> Result<u64, sqlx::Error> {
    product_repository::delete_product(pool, id).await
}
// Path: src/main.rs