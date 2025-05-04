use actix_web::{get, web::Html};
use askama::Template;
use super::super::error::Error;
//index
//search page
pub mod templates;


#[get("/")]
pub async fn index() -> Result<Html, Error> {
    Ok(Html::new(templates::IndexTemplate {}.render()?))
}