use serde::{Serialize, Deserialize};


// User Auth Types

pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub enum LoginStatus {
    Success,
    WrongPassword,
    UserNameOrPasswordNotFound,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginResponse {
    status: LoginStatus,
    sessionkey: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserAccountRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub enum CreateUserAccountStatus {
    Success,
    UsernameAlreadyExists,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserAccountResponse {
    pub status: CreateUserAccountStatus,
}

// Database Types

#[derive(Serialize, Deserialize)]
pub struct UserCredential {
    pub username: String,
    pub password: String,
    // TODO:
    // previous session keys sessionkey: String
}
