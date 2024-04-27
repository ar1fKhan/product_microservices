use sqlx::MySqlPool;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(sqlx::FromRow)]

pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f32,
}

