use tokio::sync::mpsc;

use crate::models::Product;
pub mod stockx;

use crate::scraping::error::Error;

#[async_trait::async_trait]
pub trait ProductScraping {
    fn base_search_url() -> String;
    async fn parse_product_element(element: String) -> Result<Product, Error>;
    async fn stream_product_search( c: fantoccini::Client, term: &str, limit: usize) -> Result<mpsc::Receiver<Result<Product, Error>>, Error>;
    async fn select_price( element: String) -> Result<f64, Error>;
    async fn select_title( element: String) -> Result<String, Error>;
    async fn select_image_url( element: String) -> Result<String, Error>;
    async fn select_product_url( element: String) -> Result<String, Error>;
}