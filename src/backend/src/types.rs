use serde::{Serialize, Deserialize};


// User Auth Types

pub struct UserLoginRequest {
    pub username: String,
    pub password: String,
}

pub enum LoginStatus {
    Success,
    WrongPassword,
    UserNameOrPasswordNotFound,
}

pub struct UserLoginResponse {
    status: LoginStatus,
    sessionkey: String,
}

pub struct CreateUserAccountRequest {
    username: String,
    password: String,
}

pub enum CreateUserAccountStatus {
    Success,
    UserNameAlreadyExists,
}

pub struct CreateUserAccountResponse {
    status: CreateUserAccountStatus,
    sessionkey: String, 
}

// Database Types

#[derive(Serialize, Deserialize)]
pub struct UserCredential {
    pub username: String,
    pub password: String,
    // TODO:
    // previous session keys sessionkey: String
}
