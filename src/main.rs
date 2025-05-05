use actix_web::{get, web, App, HttpServer};
use item_finder::{scraping::{client::default_client, product_scraping::{infra::ProductScraping, ProductSearch}}, web::handlers::{api::search_products::search_stream, views::{index, search_view}}};

// This struct represents state
struct AppState {
    app_name: String,
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // let mut item_reciever = ProductSearch::default("jordan".to_string())
    //     .stream_search(default_client().await.unwrap())
    //     .await
    //     .unwrap();

    // while let Some(item) = item_reciever.recv().await {
    //     match item {
    //         Ok(product) => {
    //             println!("Product: {:?}", product);
    //         }
    //         Err(e) => {
    //             eprintln!("Error: {:?}", e);
    //         }
    //     }
    // }
    

    // Ok(())
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(actix_files::Files::new("/static", "./static"))
            .service(index)
            .service(search_view)
            .service(search_stream)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}