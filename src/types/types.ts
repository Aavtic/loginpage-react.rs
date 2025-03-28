export const SERVER_IP = "192.168.2.165"
export const SERVER_PORT = 8081;

export interface UserLoginRequest {
    username: string,
    password: string,
}

export enum LoginStatus {
    Success,
    WrongPassword,
    UserNameOrPasswordNotFound,
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
    Success = "Status",
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

