use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::derive::{Display, Error};
pub mod web;
pub mod scraping;
pub mod models;

#[derive(Debug, derive_more::From, Display, Error)]
pub enum Error {

    #[from]
    #[display("Internal server error")]
    Actix(actix_web::Error),

    #[display("{message}")]
    BadRequest {
        message: String,
    }

}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}