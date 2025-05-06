use tokio::sync::mpsc;

use crate::models::Product;
use crate::scraping::error::Error;


#[async_trait::async_trait]
pub trait ProductScraping: Send { 
    fn base_search_url(&self) -> String;
    fn store_name(&self) -> String;
    fn store_color(&self) -> String;
    async fn parse_product_element(&self, element: String) -> Result<Product, Error>;
    async fn stream_product_search(&self, sender: mpsc::Sender<Result<Product, Error>>, c: fantoccini::Client, term: &str, limit: usize) -> Result<(), Error>;
    async fn select_price(&self, element: String) -> Result<f64, Error>;
    async fn select_title(&self, element: String) -> Result<String, Error>;
    async fn select_image_url(&self, element: String) -> Result<String, Error>;
    async fn select_product_url(&self, element: String) -> Result<String, Error>;

}