use actix_web::{web, App, HttpServer};
use item_finder::web::handlers::{api::search_products::search_stream, views::{index, search_view}};
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_log::LogTracer;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
// This struct represents state
struct AppState {
    _app_name: String,
}


/// Compose multiple layers into a `tracing`'s subscriber.
pub fn get_subscriber(
    name: String,
    env_filter: String
) -> impl Subscriber + Send + Sync {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or(EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(
        name.into(),
        std::io::stdout
    );
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("app".into(), "info".into());
    init_subscriber(subscriber);
    
    tracing::info!("Starting server...");
    HttpServer::new(|| {

        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                _app_name: String::from("Item Finder"),
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