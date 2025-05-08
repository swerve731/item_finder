use actix_web::{get, web::Html};
use askama::Template;
use crate::scraping::product_scraping::ProductSearch;

use super::super::error::Error;
//index
//search page
pub mod templates;


#[get("/")]
pub async fn index() -> Result<Html, Error> {
    Ok(Html::new(templates::IndexTemplate.render()?))
}

#[get("/search")]
pub async fn search_view() -> Result<Html, Error> {
    let stores_names = ProductSearch::default_scrapers()
        .iter()
        .map(|s| s.store_name())
        .collect::<Vec<String>>();

    Ok(Html::new(templates::SearchTemplate
        {
            store_names: stores_names
        }.render()?))
}

