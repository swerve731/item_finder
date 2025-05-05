use actix_web::{post, web::Bytes, HttpResponse, Responder};
use tokio_stream::wrappers::ReceiverStream;

use crate::{scraping::product_scraping::ProductSearch, Error};

use futures::stream::StreamExt;

#[post("/search")]
async fn search_stream() -> Result<impl Responder, Error> {
    let search = ProductSearch::default("jordans".to_string())
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
                    return Ok(Bytes::from(item))
                }
                Err(e) => {
                    // Handle error
                    eprintln!("Error: {:?}", e);
                    return Err(e.to_string())
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