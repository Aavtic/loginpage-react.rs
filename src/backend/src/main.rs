use axum::{routing::Router, routing::{post, options}};
use axum::response::Response;
use axum::{
    http::StatusCode,
    body::Body,
};
use axum::extract::{Json, State};
use axum::debug_handler;
use tower_http::cors::{CorsLayer, Any};
use http::{Method, header::{CONTENT_TYPE, ACCEPT}};
use serde_json;
use uuid::Uuid;


pub mod database;
use database::mongo_funcs::MongoClient;

pub mod types;
use types::{
    CreateUserAccountRequest, 
    CreateUserAccountResponse,
    CreateUserAccountStatus,
    UserCredential,
};

const USERS_COLLECTION: &str = "users_collection";
const USERS_DATABASE: &str = "TestDB";


#[debug_handler]
async fn create_account(State(mongo_client): State<MongoClient>, Json(create_account_req): Json<CreateUserAccountRequest>) -> Json<CreateUserAccountResponse>{
    println!("LOG:Request to create account:{}", serde_json::to_string(&create_account_req).unwrap());
    let user_creds = UserCredential {
        username: create_account_req.username.clone(),
        password: create_account_req.password.clone(),
    };

    let status: CreateUserAccountStatus;

    if !mongo_client.check_user_exists(USERS_DATABASE, USERS_COLLECTION, &create_account_req.username).await {
        mongo_client.insert_user(USERS_DATABASE, USERS_COLLECTION, user_creds).await;
        println!("LOG: Created user account!\nUser: {}, Password: {}", create_account_req.username, "*".repeat(create_account_req.password.len()));
        status = CreateUserAccountStatus::Success;
    } else {
        println!("LOG: User Creation Denied, User Already exists");
        status = CreateUserAccountStatus::UsernameAlreadyExists;
    }

    match status {
        CreateUserAccountStatus::Success => {
            return Json(CreateUserAccountResponse {
                status,
            });
        },
        CreateUserAccountStatus::UsernameAlreadyExists => {
            return Json(CreateUserAccountResponse {
                status,
            });
        },
        
    }
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
    let mongo_client = MongoClient::connect().await;

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE, ACCEPT]);

    let preflight_route= Router::new()
        .route("/", options(preflight_response));

    let user_api_routes = Router::new()
        .route("/create_account", post(create_account))
        .with_state(mongo_client);

    let app = Router::new()
        .nest("/users/api/", preflight_route)
        .nest("/users/api/", user_api_routes)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
