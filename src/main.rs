use actix_web::{web,App, HttpServer};
use sqlx::MySqlPool;
use dotenv::dotenv;
use std::env;


mod controllers;
mod models;
mod services;
mod repositories;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/products", web::get().to(controllers::product_controller::get_products))
            .route("/products/{id}", web::get().to(controllers::product_controller::get_product_by_id))
            .route("/products", web::post().to(controllers::product_controller::create_product))
            .route("/products/{id}", web::put().to(controllers::product_controller::update_product))
            .route("/products/{id}", web::delete().to(controllers::product_controller::delete_product))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
