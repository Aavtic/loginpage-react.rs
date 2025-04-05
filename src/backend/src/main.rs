use axum::{routing::Router, routing::{get, post, options}};
use axum::response::{Response, IntoResponse};
use axum::{
    http::{StatusCode, HeaderValue},
    body::Body,
};
use axum::extract::{Json, State, Path};
use axum::debug_handler;
use tower_http::cors::CorsLayer;
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
    UserLoginRequest,
    UserLoginResponse,
    LoginStatus,
    UserCredential,
    UserDetailsPublic,
    UserSessionStatus,
};

const USERS_COLLECTION: &str = "users_collection";
const USERS_DATABASE: &str = "TestDB";

const FRONTEND: &str = "http://192.168.219.165:5173";

async fn validate_user(State(mongo_client): State<MongoClient>, Path(session_key): Path<String>) -> Json<UserDetailsPublic> {
    let status: UserSessionStatus;
    let username: String;
    if let Some(name) = mongo_client.validate_key(USERS_DATABASE, USERS_COLLECTION, &session_key).await {
        status = UserSessionStatus::Ok;
        username = name;
    } else {
        status = UserSessionStatus::LoginRequired;
        username = String::new();
    }

    return Json(
        UserDetailsPublic {
            status,
            username,
        }
    );
}


#[debug_handler]
async fn user_login(State(mongo_client): State<MongoClient>, Json(login_request): Json<UserLoginRequest>) -> impl IntoResponse {
    println!("LOG:Request to Login:{}", serde_json::to_string(&login_request).unwrap());
    let status: LoginStatus;
    if !mongo_client.check_user_exists(USERS_DATABASE, USERS_COLLECTION, &login_request.username).await {
        status = LoginStatus::UserNameOrPasswordNotFound;
    } else {
        if let Some(password) = mongo_client.get_password(USERS_DATABASE, USERS_COLLECTION, &login_request.username).await {
            if password == login_request.password {
                status = LoginStatus::Success;
            } else {
                status = LoginStatus::WrongPassword;
            }
        } else {
            status = LoginStatus::UserNameOrPasswordNotFound;
        }
    }

    match status {
        LoginStatus::Success => {
            println!("LOG: Login Successful for {}", login_request.username);
            let sessionkey = Uuid::new_v4().to_string();
            // Update db with session key.
            mongo_client.add_session_key(USERS_DATABASE, USERS_COLLECTION, &sessionkey, &login_request.username).await; 
            // Set a permanent cookie with session key which expires in 5 days.
            let cookie_header = format!("sessionkey={}; Max-Age=432000; Path=/", sessionkey);
            return (
                [(http::header::SET_COOKIE, cookie_header)],
                Json(UserLoginResponse {
                    status,
                    sessionkey,
                }));
        },
        LoginStatus::WrongPassword => {
            println!("LOG: Wrong password attempt for {}", login_request.username);
            return (
                [(http::header::SET_COOKIE, "".to_string())],
                Json(UserLoginResponse {
                    status,
                    sessionkey: "".to_string(),
                }));
        },
        LoginStatus::UserNameOrPasswordNotFound => {
            println!("LOG: Wrong username or password attempt for {}", login_request.username);
            return (
                [(http::header::SET_COOKIE, "".to_string())],
                Json(UserLoginResponse {
                    status,
                    sessionkey: "".to_string(),
                }));
        },
    }
}

#[debug_handler]
async fn create_account(State(mongo_client): State<MongoClient>, Json(create_account_req): Json<CreateUserAccountRequest>) -> Json<CreateUserAccountResponse>{
    println!("LOG:Request to create account:{}", serde_json::to_string(&create_account_req).unwrap());
    let user_creds = UserCredential {
        username: create_account_req.username.clone(),
        password: create_account_req.password.clone(),
        session_key_pool: Vec::<String>::new(),
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

    return Json(CreateUserAccountResponse{status});
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
        .allow_origin(FRONTEND.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::POST, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE, ACCEPT])
        .allow_credentials(true);

    let preflight_route= Router::new()
        .route("/", options(preflight_response));

    let user_api_routes = Router::new()
        .route("/create_account", post(create_account))
        .route("/login", post(user_login))
        .route("/validate_session/{session_key}", get(validate_user))
        .with_state(mongo_client);

    let app = Router::new()
        .nest("/users/api/", preflight_route)
        .nest("/users/api/", user_api_routes)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
