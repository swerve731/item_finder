use crate::{models::Product, scraping:: error::Error};
use fantoccini::{elements::Element, Locator};
use scraper::{Html, Selector};

use super::super::infra::ProductScraping;


#[derive(Clone, Copy)]
pub struct EbayScraper;

#[async_trait::async_trait]
impl ProductScraping for EbayScraper {
    fn format_search_term_url(&self, term: String) -> String{
        format!("https://www.ebay.com/sch/i.html?_nkw={}", term)
    }
    fn store_name(&self) -> String{
        "Ebay".to_string()
    }
    fn store_color(&self) -> String{
        "#E53238".to_string()
    }

    

    async fn select_product_elements(&self, c: fantoccini::Client) -> Result<Vec<Element>, Error>{
        let elements = c.find_all(Locator::Css(r#"li.s-item"#))
            .await?;

        Ok(elements)
    }

    async fn parse_product_element(&self, element: String) -> Result<Product, Error>{
        let title = self.select_title(element.clone()).await?;
        let price = self.select_price(element.clone()).await?;
        let image_url = self.select_image_url(element.clone()).await?;
        let product_url = self.select_product_url(element.clone()).await?;

        let product = Product {
            title,
            price,
            image_url,
            product_url,
            store_name: self.store_name(),
            store_color: self.store_color(),
        };

        Ok(product)
    }
    async fn select_price(&self, element: String) -> Result<f64, Error>{
        let element = Html::parse_fragment(&element);
        let price_selector = Selector::parse(r#"span.s-item__price"#)?;
            
        let price_string: String = element.select(&price_selector)
            .next()
            .ok_or(Error::NotFound(format!("Ebay price not found for element: {:?}", element)))?
            .text()
            .collect::<String>();

            let price_string =  if let Some(i) = price_string.find("to") {
                let s = price_string[0..i].into();
                s
            } else {
                price_string
            };

            let parsed_price = price_string
                .replace(['$', ',', '£', '€', ' '], "")

                .trim()
                .parse::<f64>()
                .map_err(|e| Error::WrongDataType(format!("Could not parse the ebay price element: {e:?}")))?;

        Ok(parsed_price)
    }

    async fn select_title(&self, element: String) -> Result<String, Error>{
        let element = Html::parse_fragment(&element);

        let title_selector = Selector::parse(r#"div.s-item__title"#)?;
        let title: String = element.select(&title_selector)
            .next()
            .ok_or(Error::NotFound(format!("Ebay title not found for element: {:?}", element)))?
            .text()
            .collect::<String>();

        Ok(title)
    }
    async fn select_image_url(&self, element: String) -> Result<String, Error>{
        let element = Html::parse_fragment(&element);

        let image_selector = Selector::parse(r#"img"#)?;
        let image_url: String = element.select(&image_selector)
            .next()
            .ok_or(Error::NotFound(format!("Ebay image url not found for element: {:?}", element)))?
            .value()
            .attr("src")
            .ok_or(Error::NotFound(format!("Ebay image url not found for element: {:?}", element)))?
            .to_string();

        Ok(image_url)
    }
    async fn select_product_url(&self, element: String) -> Result<String, Error>{
        let element = Html::parse_fragment(&element);

        let url_selector = Selector::parse(r#"a.s-item__link"#)?;
        let product_url: String = element.select(&url_selector)
            .next()
            .ok_or(Error::NotFound(format!("Ebay product url not found for element: {:?}", element)))?
            .value()
            .attr("href")
            .ok_or(Error::NotFound(format!("Ebay product url not found for element: {:?}", element)))?
            .to_string();

        Ok(product_url)
    }












    // this is the originl implementation to scrap the products and has since been replaced by ProductSearch using the ProductScraping trait
    // async fn stream_product_search(&self, sender: mpsc::Sender<Result<Product, Error>>, term: String, limit: usize) -> Result<(), Error>{
    //     let term = term.clone();
       
    //     let s = self.clone();
    //     tokio::spawn(
    //         async move {

    //             let term = term.replace(" ", "+");
    //             let url = s.format_search_term_url(term);
    //             println!("ebay url: {:?}", url);

    //             let c = start_client()
    //                 .await
    //                 .unwrap();
    //             println!("ebay client");
    //             c.goto(&url)
    //                 .await
    //                 .unwrap();
    //             println!("ebay goto");
    //             let product_elements = c.find_all(Locator::Css(r#"li.s-item"#))
    //                 .await
    //                 .unwrap();
    //             let mut i = 0;
        
    //             while product_elements.len() > i && i < limit{
    //                 let raw_element = product_elements[i]
    //                     .html(true)
    //                     .await;


    //                 // sleep for effect
    //                 // tokio::time::sleep(tokio::time::Duration::from_millis(2)).await;

    //                 match raw_element {
    //                     Ok(element) => {
    //                         let product = s.parse_product_element(element.clone()).await;
    //                         println!("ebay");
    //                         match product {
    //                             Ok(product) => {

    //                                 // ebay has invisble elements or smthn
    //                                 if product.title == "Shop on eBay" {
    //                                     i += 1;
    //                                     continue;
    //                                 }
                                    
    //                                 sender.send(Ok(product))
    //                                     .await
    //                                     .map_err(|e| format!("Error sending product to sender {:?}", e))
    //                                     .unwrap();

    //                             }
    //                             Err(err) => {
    //                                 sender.send(Err(err))
    //                                     .await
    //                                     .map_err(|e| format!("Error sending error to sender {:?}", e))
    //                                     .unwrap();
    //                             }
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