use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub store_name: String,
    pub title: String,
    pub price: f64,
    pub image_url: String,
    pub product_url: String,
    pub store_color: String,
}