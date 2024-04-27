use sqlx::MySqlPool;

use crate::models::product::Product;
pub struct ProductRepository;

impl ProductRepository {
    pub async fn get_products(pool: &MySqlPool) -> Result<Vec<Product>, sqlx::Error> {
        let products = sqlx::query_as::<_, Product>("SELECT * FROM products")
            .fetch_all(pool)
            .await?;

        Ok(products)
    }

    pub async fn get_product_by_id(pool: &MySqlPool, id: i32) -> Result<Product, sqlx::Error> {
        let product = sqlx::query_as::<_, Product>("SELECT * FROM products WHERE id = ?")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(product)
    }

    pub async fn create_product(pool: &MySqlPool, product: Product) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("INSERT INTO products (name, description, price) VALUES (?, ?, ?)")
            .bind(product.name)
            .bind(product.description)
            .bind(product.price)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn update_product(pool: &MySqlPool, id: i32, product: Product) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("UPDATE products SET name = ?, description = ?, price = ? WHERE id = ?")
            .bind(product.name)
            .bind(product.description)
            .bind(product.price)
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn delete_product(pool: &MySqlPool, id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM products WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }
}

// Path: src/repositories/product_repository.rs

// Path: src/routes/product_routes.rs