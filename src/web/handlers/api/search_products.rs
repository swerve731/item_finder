use actix_web::{post, web::{self, Bytes}, HttpResponse, Responder};
use tokio_stream::wrappers::ReceiverStream;

use crate::{scraping::product_scraping::ProductSearch, Error};

use futures::stream::StreamExt;

#[derive(serde::Deserialize)]
struct SearchForm {
    term: String,
}

#[post("/search")]
async fn search_stream(web::Form(form): web::Form<SearchForm>) -> Result<impl Responder, Error> {
    let search = ProductSearch::default(form.term)
        .stream_search(crate::scraping::client::default_client().await.unwrap())
        .await
        .unwrap();

    let stream = ReceiverStream::new(search)
        .map(|item| {
            match item {
                Ok(product) => {
                    // Convert product to JSON
                    println!("Product: {:?}", product);
                    let item = serde_json::to_string(&product).unwrap();
                    // Convert item to bytes
                    return Ok::<actix_web::web::Bytes, Error>(Bytes::from(item))
                }
                Err(e) => {
                    return Ok::<actix_web::web::Bytes, Error>(Bytes::from(
                        format!("{{\"error\": \"{}\"}}", e.to_string())
                    ));
                    // Handle error
                    // eprintln!("Error: {:?}", e);
                    // let json_error: String  = serde_json::from_str(&format!("{{\"error\": \"{}\"}}", e.to_string())).unwrap();
                    // return Ok::<actix_web::web::Bytes, Error>(Bytes::from(
                    //     json_error
                    // ));
                }
            }

        });


    // todo!("stream the response i think it has to be in bytes");
    
    Ok(
        HttpResponse::Ok()
            .content_type("text/event-stream")
            .streaming(stream)
    )
}