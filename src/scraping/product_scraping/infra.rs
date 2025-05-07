use fantoccini::elements::Element;

use crate::models::Product;
use crate::scraping::error::Error;


#[async_trait::async_trait]
pub trait ProductScraping: Send { 
    fn format_search_term_url(&self, term: String) -> String;
    fn store_name(&self) -> String;
    fn store_color(&self) -> String;
    async fn parse_product_element(&self, element: String) -> Result<Product, Error>;
    async fn select_price(&self, element: String) -> Result<f64, Error>;
    async fn select_title(&self, element: String) -> Result<String, Error>;
    async fn select_image_url(&self, element: String) -> Result<String, Error>;
    async fn select_product_url(&self, element: String) -> Result<String, Error>;
    async fn select_product_elements(&self, c: fantoccini::Client) -> Result<Vec<Element>, Error>;


    // original way of streaming the product scrape but I now use the trait functions in the ProductSearch 
    // async fn stream_product_search(&self, sender: mpsc::Sender<Result<Product, Error>>, term: String, limit: usize) -> Result<(), Error>;

}