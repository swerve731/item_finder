use crate::{models::Product, scraping::error::Error};
use fantoccini::{elements::Element, Locator};
use scraper::{Html, Selector};

use super::super::infra::ProductScraping;


#[derive(Clone, Copy)]
pub struct StockxScraper;


#[async_trait::async_trait]
impl ProductScraping for StockxScraper {
    fn format_search_term_url(&self, term: String) -> String {
        format!("https://stockx.com/search?s={}", term)
    }

    fn store_color(&self) -> String {
        "#006340".to_string()
    }

    fn store_name(&self) -> String {
        "StockX".to_string()
    }

    async fn select_product_elements(&self, c: fantoccini::Client) -> Result<Vec<Element>, Error> {
        let elements = c.find_all(Locator::Css(r#"div[data-testid="productTile"]"#))
            .await?;

        Ok(elements)
    }
    
    async fn parse_product_element(&self,element: String) -> Result<Product, Error> {
        let title = self.select_title(element.clone()).await?;
        let price = self.select_price(element.clone()).await?;
        // dbg!(raw_element.clone());
        
        let image_url = self.select_image_url(element.clone()).await?;
        let product_url = self.select_product_url(element.clone()).await?;           
        
        Ok(Product {
            title,
            price,
            image_url,
            product_url,
            store_name: self.store_name(),
            store_color: self.store_color(),
        })
    }

    async fn select_price(&self,element: String) -> Result<f64, Error> {
        // dbg!(element.clone());
        let element = Html::parse_fragment(&element);

        let price_selector = Selector::parse(r#"p[data-testid="product-tile-lowest-ask-amount"]"#)?;
        // dbg!(price_selector.clone());
        let price_string: String = element.select(&price_selector)
            .next()
            .ok_or(Error::NotFound(format!("StockX price not found for element: {:?}", element)))?
            .text()
            .collect::<String>();
        // dbg!(price_string.clone());

        let parsed_price: f64 = price_string
            .replace("$", "")
            .replace(",", "")
            .parse()
            .map_err(|e| Error::WrongDataType(format!("Could not parse the stockX price element: {:?}\n\n Parsing Error: {:?}", price_string, e)))?;
        Ok(parsed_price)
    }   

    async fn select_title(&self,element: String) -> Result<String, Error> {
        let element = Html::parse_fragment(&element);

        let title_selector = Selector::parse(r#"p[data-testid="product-tile-title"]"#)?;
        let title: String = element.clone().select(&title_selector)
            .next() 
            .ok_or(Error::NotFound(format!("StockX title not found for element: {:?}", element)))?
            .text()
            .collect::<String>();
        Ok(title)
    }

    async fn select_image_url(&self,element: String) -> Result<String, Error> {
        
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
            .replace(" 1x", "")
            // change the width and height for heigher resolutions
            // 16:9
            .replace("w=140", "w=280")
            .replace("h=75", "h=150");

  

        

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



    // this is the originl implementation to scrap the products and has since been replaced by ProductSearch using the ProductScraping trait
    // async fn stream_product_search(&self, sender: mpsc::Sender<Result<Product, Error>>, term: String, limit: usize ) -> Result<(), Error> {
    //     let term = term.clone();
    //     let s = self.clone();
    //     tokio::spawn(

    //         async move {
    //             let term = term.replace(" ", "+");

    //             //"&available-now=true" makes sure all the products are available wich means they will all have a price
    //             // this fixes the issue of some products not having a price
    //             let url = s.format_search_term_url(term).clone();   

    //             println!("stockx url: {:?}", url);

    //             let c = start_client()
    //                 .await
    //                 .unwrap();
    //             println!("stockx client");
    //             c.goto(&url)
    //                 .await
    //                 .unwrap();
    //             println!("stockx goto");
    //             let product_elements = c.find_all(Locator::Css(r#"div[data-testid="productTile"]"#))
    //                 .await
    //                 .unwrap();
    //             let mut i = 0;
    //             // println!()

    //             while product_elements.len() > i && i < limit{
    //                 let raw_element = product_elements[i]
    //                     .html(true)
    //                     .await;

    //                 // dbg!(&raw_element);

    //                 // sleep for effect

    //                 match raw_element {
    //                     Ok(element) => {
    //                         let product = s.parse_product_element(element.clone()).await;
                            
    //                         // dbg!(&product);
    //                         println!("stockx");
    //                         // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    //                         if let Err(send_err) = sender.send(product).await {
    //                             eprintln!("Failed to send product: {:?}", send_err);
    //                             i+=1;
    //                             continue; 
    //                         }
                            
                            
    //                         i += 1;
    //                     }
    //                     Err(err) => {
    //                         sender.send(Err(Error::FantocciniCmd(err)))
    //                             .await
    //                             .map_err(|e| format!("Error sending error to sender {:?}", e))
    //                             .unwrap();

    //                         i += 1;
    //                     }
    //                 }
                    
                    
                    
    //             };
    //             drop(sender);
    //         }
    //     );
        
    //     Ok(())
    
    // }

}

