use axum::{Json, Router, routing::get};
use core::str;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct Thing {
    id: u32,
    name: String,
}

async fn list_things() -> Json<Vec<Thing>> {
    Json(vec![
        Thing {
            id: 1,
            name: "Thing 1".into(),
        },
        Thing {
            id: 2,
            name: "Thing 2".into(),
        },
    ])
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/list_things", get(list_things));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Listening on port {}", addr.port());

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
