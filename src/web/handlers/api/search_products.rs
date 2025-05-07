use actix_web::{post, web::{self, Bytes}, HttpResponse, Responder};
use tokio_stream::wrappers::ReceiverStream;

use crate::{scraping::product_scraping::ProductSearch, Error};

use futures::stream::StreamExt;

#[derive(serde::Deserialize)]
struct SearchForm {
    term: String,
}

#[post("/search")]
async fn search_stream(web::Json(form): web::Json<SearchForm>) -> Result<impl Responder, Error> {
    println!("Search term: {}", form.term);
    let search = ProductSearch::default(form.term)
        .stream_search()
        .await
        .unwrap();

    let stream = ReceiverStream::new(search)
        .map(|item| {
            match item {
                Ok(product) => {
                    // Convert product to JSON
                    println!("product");
                    // println!("product returned from {:?}", product.store_name);
                    let item = serde_json::to_string(&product).unwrap();
                    println!("{:?}", item);
                    // Convert item to bytes
                    return Ok::<actix_web::web::Bytes, Error>(Bytes::from(item))
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    return Ok::<actix_web::web::Bytes, Error>(Bytes::from(

                        format!("{{\"error\": \"{}\"}}", e.to_string())
                    ))
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