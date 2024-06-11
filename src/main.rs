#[macro_use]
mod utils;
mod config;
mod models;
mod logger;
use axum::{http::StatusCode, Router};
use axum_auto_routes::route;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use utils::WithState;

lazy_static::lazy_static! {
    pub static ref ROUTE_REGISTRY: Mutex<Vec<Box<dyn WithState>>> = Mutex::new(Vec::new());
}

#[tokio::main]
async fn main() {
    println!("quest_server: starting v{}", env!("CARGO_PKG_VERSION"));
    let conf = config::load();
    let logger = logger::Logger::new(&conf.watchtower);
    let client_options = ClientOptions::parse(&conf.database.connection_string)
        .await
        .unwrap();

    let client = reqwest::Client::builder().build().unwrap();
    let shared_state = Arc::new(models::AppState {
        conf: conf.clone(),
        db: Client::with_options(client_options)
            .unwrap()
            .database(&conf.database.name),
        logger,
    });
    if shared_state
        .db
        .run_command(doc! {"ping": 1}, None)
        .await
        .is_err()
    {
        println!("error: unable to connect to database");
        return;
    } else {
        println!("database: connected")
    }

    let cors = CorsLayer::new().allow_headers(Any).allow_origin(Any);
    let app = ROUTE_REGISTRY.lock().unwrap().clone().into_iter().fold(
        Router::new().with_state(shared_state.clone()).layer(cors),
        |acc, r| acc.merge(r.to_router(shared_state.clone())),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], conf.server.port));
    println!("server: listening on http://0.0.0.0:{}", conf.server.port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

#[route(get, "/")]
async fn root() -> (StatusCode, String) {
    (
        StatusCode::ACCEPTED,
        format!("server v{}", env!("CARGO_PKG_VERSION")),
    )
}
