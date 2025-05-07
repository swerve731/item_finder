
use crate::models::Product;
use crate::scraping::client::start_client;
use crate::scraping::error::Error;

use infra::ProductScraping;
pub mod scrapers;

use scrapers::stockx::StockxScraper;

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
                        let sender = tx.clone();
                        let term = self.term.clone();
                        let limit = 30;

                        tokio::spawn(
                            async move {

                                let term = term.replace(" ", "+");
                                let url = scraper.format_search_term_url(term);
                                println!("scraping store: {:?}", scraper.store_name());
                                let c = start_client()
                                    .await
                                    .unwrap();
                                c.goto(&url)
                                    .await
                                    .unwrap();
                                println!("goto");

                                let product_elements = scraper.select_product_elements(c)
                                    .await
                                    .unwrap();

                                println!("product_elements");

                                let mut i = 0;
                                while product_elements.len() > i && i < limit{
                                    let raw_element = product_elements[i]
                                        .html(true)
                                        .await;


                                    // sleep for effect
                                    // tokio::time::sleep(tokio::time::Duration::from_millis(2)).await;
                                    match raw_element {
                                        Ok(element) => {
                                            let product = scraper.parse_product_element(element.clone()).await;
                                            match product {
                                                Ok(product) => {

                                                    // ebay has invisble elements or smthn
                                                    if product.title == "Shop on eBay" {
                                                        i += 1;
                                                        continue;
                                                    }
                                                    
                                                    // println!("product: {:?}", product);
                                                    sender.send(Ok(product))
                                                        .await
                                                        .map_err(|e| format!("Error sending product to sender {:?}", e))
                                                        .unwrap();

                                                }
                                                Err(err) => {
                                                    println!("error: {:?}", err);
                                                    sender.send(Err(err))
                                                        .await
                                                        .map_err(|e| format!("Error sending error to sender with store {:?}\n and product: {:?} \n and err: {:?} ", scraper.store_name(), element, e))
                                                        .unwrap();
                                                }
                                            }
                                        
                                            i += 1;
                                        }
                                        Err(err) => {
                                            sender.send(Err(Error::FantocciniCmd(err)))
                                                .await
                                                .map_err(|e| format!("Error sending error to sender {:?}", e))
                                                .unwrap();

                                            i += 1;
                                        }
                                    }
                                    
                                    
                                    
                                };

                                // let result = scraper
                                //     .stream_product_search( tx.clone(), term, self.limit.clone());
    
                                // let result = result.await;
                                // if let Err(e) = result {
                                //     eprintln!("Error: {:?}", e);
                                // }  
                            }
                        );
                                              
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
        let limit = 10; 
        let scrapers = Self::default_scrapers();

        Self {
            term,
            limit,
            scrapers,
        }
    }

    pub fn default_scrapers() -> Vec<Box<dyn ProductScraping>> {
        vec![
            Box::new(scrapers::ebay::EbayScraper),

            Box::new(StockxScraper),
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



