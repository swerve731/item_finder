use crate::{models::Product, scraping::error::Error};
use fantoccini::Locator;
use scraper::{Html, Selector};

use super::ProductScraping;

pub struct StockxScraper;

#[async_trait::async_trait]
impl ProductScraping for StockxScraper {
    fn base_search_url(&self) -> String {
        "https://stockx.com/search?s=".to_string()
    }

    async fn search_products(&self,c: fantoccini::Client, term: &str, limit: usize ) -> Result<Vec<Product>, Error> {
        let url = self.base_search_url() + term;

        c.goto(&url).await?;
        let product_elements = c.find_all(Locator::Css(r#"div[data-testid="productTile"]"#)).await?;
        let mut i = 0;

        let mut products: Vec<Product> = Vec::new();

        while product_elements.len() > i && i < limit{
            let raw_element = product_elements[i].html(true).await?;

            let title = self.select_title(raw_element.clone()).await?;
            let price = self.select_price(raw_element.clone()).await?;
            // dbg!(raw_element.clone());
            
            let image_url = self.select_image_url(raw_element.clone()).await?;
            let product_url = self.select_product_url(raw_element.clone()).await?;           


            products.push(Product {
                title,
                price,
                image_url,
                product_url,
            });

            i+=1;
        };

        Ok(products)
    
    }

    async fn select_price(&self, element: String) -> Result<f64, Error> {
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

    async fn select_title(&self, element: String) -> Result<String, Error> {
        let element = Html::parse_fragment(&element);

        let title_selector = Selector::parse(r#"p[data-testid="product-tile-title"]"#)?;
        let title: String = element.clone().select(&title_selector)
            .next() 
            .ok_or(Error::NotFound(format!("StockX title not found for element: {:?}", element)))?
            .text()
            .collect::<String>();
        Ok(title)
    }

    async fn select_image_url(&self, element: String) -> Result<String, Error> {
        
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

    async fn select_product_url(&self, element: String) -> Result<String, Error> {
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

