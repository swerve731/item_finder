use crate::{models::Product, scraping::error::Error};
use fantoccini::Locator;
use scraper::{Html, Selector};
use tokio::sync::mpsc;

use super::ProductScraping;

pub struct StockxScraper;



#[async_trait::async_trait]
impl ProductScraping for StockxScraper {
    fn base_search_url() -> String {
        "https://stockx.com/search?s=".to_string()
    }

    async fn stream_product_search(c: fantoccini::Client, term: &str, limit: usize ) -> Result<mpsc::Receiver<Result<Product, Error>>, Error> {
        let url = Self::base_search_url() + term;

        let (tx, mut rx) = mpsc::channel::<Result<Product, Error>>(1);

        c.goto(&url).await?;
        let product_elements = c.find_all(Locator::Css(r#"div[data-testid="productTile"]"#)).await?;
        let mut i = 0;

        let mut products: Vec<Product> = Vec::new();

        tokio::spawn(
            async move {
                while product_elements.len() > i && i < limit{
                    let raw_element = product_elements[i]
                        .html(true)
                        .await;

                    match raw_element {
                        Ok(element) => {
                            let product = Self::parse_product_element(element.clone()).await;
                            tx.send(product)
                                .await
                                .unwrap();
                            i += 1;
                        }
                        Err(err) => {
                            tx.send(Err(err.into()))
                                .await
                                .unwrap();
                            i += 1;
                            continue;
                        }
                    }
                    
                    
                    
                };
            }
        );
        

        Ok(rx)
    
    }


    async fn parse_product_element(element: String) -> Result<Product, Error> {
        let title = Self::select_title(element.clone()).await?;
        let price = Self::select_price(element.clone()).await?;
        // dbg!(raw_element.clone());
        
        let image_url = Self::select_image_url(element.clone()).await?;
        let product_url = Self::select_product_url(element.clone()).await?;           
        
        Ok(Product {
            title,
            price,
            image_url,
            product_url,
        })
    }

    async fn select_price(element: String) -> Result<f64, Error> {
        let element = Html::parse_fragment(&element);

        let price_selector = Selector::parse(r#"p[data-testid="product-tile-lowest-ask-amount"]"#)?;
        let price_string: String = element.select(&price_selector)
            .next()
            .ok_or(Error::NotFound(format!("StockX price not found for element: {:?}", element)))?
            .text()
            .collect::<String>();
        
        let parsed_price: f64 = price_string
            .replace("$", "")
            .parse()
            .map_err(|e| Error::WrongDataType(format!("Could not parse the stockX price element: {:?}\n\n Parsing Error: {:?}", price_string, e)))?;
        Ok(parsed_price)
    }   

    async fn select_title(element: String) -> Result<String, Error> {
        let element = Html::parse_fragment(&element);

        let title_selector = Selector::parse(r#"p[data-testid="product-tile-title"]"#)?;
        let title: String = element.clone().select(&title_selector)
            .next() 
            .ok_or(Error::NotFound(format!("StockX title not found for element: {:?}", element)))?
            .text()
            .collect::<String>();
        Ok(title)
    }

    async fn select_image_url(element: String) -> Result<String, Error> {
        
        let element = Html::parse_fragment(&element);
        let image_selector = Selector::parse(r#"img"#)?;

        //stock x returns multiple urls in the scrset
        // i split it by , and take the first item
        // the images are in the format of "url 1x, url 2x, url 3x"
        // so i remove the " 1x" from the first item

        let image_urls= element.select(&image_selector)
            .next()
            .ok_or(Error::NotFound(format!("StockX image url not found for element: {:?}", element)))?
            .value()
            .attr("srcset")
            .ok_or(Error::NotFound(format!("StockX image url not found for element: {:?}", element)))?
            .split(',')
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let image_url = image_urls[0]
            .replace(" 1x", "");

        

        Ok(image_url)
    }

    async fn select_product_url( element: String) -> Result<String, Error> {
        let element = Html::parse_fragment(&element);
        let product_selector = Selector::parse(r#"a[data-testid="productTile-ProductSwitcherLink"]"#)?;
        let product_url: String = element.select(&product_selector)
            .next()
            .ok_or(Error::NotFound(format!("StockX product url not found for element: {:?}", element)))?
            .value()
            .attr("href")
            .ok_or(Error::NotFound(format!("StockX product url not found for element: {:?}", element)))?
            .to_string();
        let product_url = format!("https://stockx.com{}", product_url);
        Ok(product_url)
    }
}

