use std::net::SocketAddr;

use axum::{Json, Router, response::IntoResponse, routing::get};
use log::info;
use serde::{Deserialize, Serialize};

use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

// a simple data we will send and receive as JSON
#[derive(Debug, Serialize, Deserialize)]
struct Message {
    content: String,
}

// Handler for GET /messages
async fn list_messages() -> impl IntoResponse {
    info!("Handling list_messages request");
    Json(vec!["Hello from the server!".to_string()])
}

// Handler for POST /messages
async fn create_message(Json(message): Json<Message>) -> impl IntoResponse {
    info!("Handling create_message request");
    Json(format!("New message: {}", message.content))
}

// allows rust code to have async feature
#[tokio::main]
async fn main() {
    // 1. Intialize the tracing + log bridging ( log bridging is using log with tracing so that we can get details like method type, path, request id)
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_practice=error,tower_http=warn"))
                .unwrap(),
        ) // try_new -> Returns a new EnvFilter from the directives in the given string, or an error if any are invalid.
        .init();

    // 2. Build our router
    let app = Router::new()
        // define route GET message
        .route("/message", get(list_messages).post(create_message))
        //3. add trace layer to automatically create and enter span.
        .layer(TraceLayer::new_for_http());

    // 4. run our axum server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
