
use axum::{routing::get, Router, response::IntoResponse, extract::Query};
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize)]
struct EventsQuery { since: Option<String> }

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/", get(root))
        .route("/events", get(events))
        .route("/merkle_root", get(merkle_root));

    let addr = SocketAddr::from(([0,0,0,0], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn health() -> impl IntoResponse { axum::Json(serde_json::json!({"status":"ok"})) }
async fn root() -> impl IntoResponse { axum::Json(serde_json::json!({"service":"provenance"})) }
async fn events(Query(_q): Query<EventsQuery>) -> impl IntoResponse {
    axum::Json(vec![serde_json::json!({"type":"agent-status","ts":"now"})])
}
async fn merkle_root() -> impl IntoResponse { axum::Json(serde_json::json!({"root":"0xdeadbeef"})) }
