use actix_web::{get, web, App, HttpServer};
use item_finder::web::handlers::views::index;

// This struct represents state
struct AppState {
    app_name: String,
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(actix_files::Files::new("/static", "./static"))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}