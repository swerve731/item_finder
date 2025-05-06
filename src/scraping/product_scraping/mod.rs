
use crate::models::Product;
use crate::scraping::error::Error;
use infra::ProductScraping;
pub mod scrapers;

use scrapers::stockx::StockxScraper;

use super::client::default_client;
pub mod infra;



// Make SearchProducts non-generic
pub struct ProductSearch {
    pub term: String,
    pub limit: usize,
    pub scrapers: Vec<Box<dyn ProductScraping>>,
}

impl ProductSearch {
    pub async fn stream_search(
        self,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<Product, Error>>, Error> {

            // Create a channel for sending products
        let (tx, rx) = tokio::sync::mpsc::channel(69);
            tokio::spawn(
                async move {
                    for scraper in self.scrapers {
                        let c = default_client()
                            .await
                            .map_err(|e| format!("Error creating client: {:?}", e))
                            .unwrap();
                        
                        let result = scraper
                            .stream_product_search(tx.clone(), c, &self.term.clone(), self.limit.clone())
                            .await;

                        if let Err(e) = result {
                            eprintln!("Error: {:?}", e);
                            continue;
                        }
                    }
                },
            );

        Ok(rx)
    }

    pub fn new(term: String,) -> Self {
        Self {
            term,
            limit: 10,
            scrapers: vec![],
        }
    }


    pub fn default(term: String) -> Self {
        let limit = 30; 
        let scrapers = Self::default_scrapers();

        Self {
            term,
            limit,
            scrapers,
        }
    }

    pub fn default_scrapers() -> Vec<Box<dyn ProductScraping>> {
        vec![
            Box::new(StockxScraper),
            Box::new(scrapers::ebay::EbayScraper),
            // Add more scrapers here
        ]
    }

    pub fn with_scraper(mut self, scraper: Box<dyn ProductScraping> ) -> Self {

        self.scrapers.push(scraper);
        self
    }
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}



