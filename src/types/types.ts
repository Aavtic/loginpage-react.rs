export const SERVER_IP = "192.168.219.165"
export const SERVER_PORT = 8081;

export interface UserLoginRequest {
    username: string,
    password: string,
}

export enum LoginStatus {
    Success = "Success",
    WrongPassword = "WrongPassword",
    UserNameOrPasswordNotFound = "UserNameOrPasswordNotFound",
}

export interface UserLoginResponse {
    status: LoginStatus,
    sessionkey: String,
}

export interface CreateUserAccountRequest {
    username: string,
    password: string,
}

export enum CreateUserAccountStatus {
    Success = "Success",
    UsernameAlreadyExists = "UsernameAlreadyExists",
    // Client Side Added. Possibly network error or server side error. used when the fetch request doesn't complete correctly.
    ServerError = "ServerError",
}

export interface CreateUserAccountResponse {
    status: CreateUserAccountStatus,
}

// Client Side Errrs
export enum ClientError {
    ServerError,
    // Mostly if fetch or some client side functions fail
    ClientCooked,
}

// Result type
export type Result<T> =
    | { ok: true; data: T }
    | { ok: false; error: ClientError};


// User Session Status

export enum UserSessionStatus {
    Ok = "Ok",
    LoginRequired = "LoginRequired",
}

// User details to be made public

export interface UserDetailsPublic {
    status: UserSessionStatus,
    username: String,
}
