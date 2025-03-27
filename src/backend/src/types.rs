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
