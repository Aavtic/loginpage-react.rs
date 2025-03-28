export const SERVER_IP = "192.168.196.165"
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
    Success,
    UserNameAlreadyExists,
}

export interface CreateUserAccountResponse {
    status: CreateUserAccountStatus,
    sessionkey: string, 
}

