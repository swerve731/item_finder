use actix_web::ResponseError;
use derive_more::{Display, From, Error};

#[derive(Debug, From, Display, Error)]
pub enum Error {

    #[from]
    #[display("Internal server error")]
    Askama(askama::Error),
}


impl ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .insert_header(actix_web::http::header::ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}