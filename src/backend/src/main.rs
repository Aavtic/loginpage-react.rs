use axum::{routing::Router, routing::{get, options}};
use axum::response::Response;
use axum::{
    http::StatusCode,
    body::Body,
};
use tower_http::cors::{CorsLayer, Any};
use http::{Method, header::{CONTENT_TYPE, ACCEPT}};


pub mod types;
use types;

async fn hello_world() -> String {
    return "XMLFuckGang".to_string();
}

async fn preflight_response() -> Response {
    let response = Response::builder()
        .status(StatusCode::NO_CONTENT)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(Body::default())
        .unwrap();
    println!("PREFLIGHT SENT!");
    return response;
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE, ACCEPT]);

    let api_routes= Router::new()
        .route("/hello_world", get(hello_world))
        .route("/", options(preflight_response));

    let app = Router::new()
        .nest("/users/api/", api_routes)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
