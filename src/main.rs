use askama::Template;
use axum::routing::get;
use rust_embed::RustEmbed;
use tokio_rusqlite::Connection;
use tower_http::compression::CompressionLayer;

mod db;

#[cfg(not(debug_assertions))]
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(debug_assertions)]
const VERSION: &str = "dev";

#[derive(RustEmbed, Clone)]
#[folder = "assets/"]
struct Assets;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[tokio::main]
async fn main() {
    let mut migrations_conn = rusqlite::Connection::open("./db.sqlite").unwrap();
    embedded::migrations::runner()
        .run(&mut migrations_conn)
        .unwrap();

    let comression_layer = CompressionLayer::new().gzip(true);

    let app = axum::Router::new()
        .route("/", get(home))
        .nest_service("/assets", axum_embed::ServeEmbed::<Assets>::new())
        .layer(comression_layer)
        .with_state(Connection::open("./db.sqlite").await.unwrap());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> HomeTemplate {
    HomeTemplate {}
}

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;
