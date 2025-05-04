use crate::models::Product;
pub mod stockx;

use crate::scraping::error::Error;

#[async_trait::async_trait]
pub trait ProductScraping {
    fn base_search_url(&self) -> String;
    async fn search_products(&self, c: fantoccini::Client, term: &str, limit: usize) -> Result<Vec<Product>, Error>;
    async fn select_price(&self, element: String) -> Result<f64, Error>;
    async fn select_title(&self, element: String) -> Result<String, Error>;
    async fn select_image_url(&self, element: String) -> Result<String, Error>;
    async fn select_product_url(&self, element: String) -> Result<String, Error>;
}