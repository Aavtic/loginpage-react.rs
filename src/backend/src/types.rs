use serde::{Serialize, Deserialize};


// User Auth Types

#[derive(Serialize, Deserialize)]
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
    pub status: LoginStatus,
    // Remove this after setting the sessionkey by using Set-Cookie Header.
    pub sessionkey: String,
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
    pub session_key_pool: Vec<String>,
}


// User Session Status

#[derive(Serialize, Deserialize)]
pub enum UserSessionStatus {
    Ok,
    LoginRequired,
}

// User details to be made public

#[derive(Serialize, Deserialize)]
pub struct UserDetailsPublic {
    pub status: UserSessionStatus,
    pub username: String,
    // TODO: Add more customized details relavant to each user.
}
