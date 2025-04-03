import { SERVER_IP, SERVER_PORT, ClientError, Result} from "../types/types";
import { CreateUserAccountRequest, CreateUserAccountResponse, CreateUserAccountStatus } from "../types/types";


export async function create_post<PostType, ExpectedType>(url: string, data: PostType, credentials: boolean): Promise<Result<ExpectedType>> {
    try {
        let headers: RequestInit;
        if (credentials) {
            headers = {
                "method": "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                credentials: 'include',
                body: JSON.stringify(data)
           }
        } else {
            headers = {
                "method": "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(data)
           }
       }
       const response = await fetch (url, headers);
       if (!response.ok) {
           return  {
               ok: true,
               data: await response.json() as ExpectedType,
           }
       }

       return  {
           ok: true,
           data: await response.json() as ExpectedType,
       }
    } catch (e) {
        return {
            ok: false,
            error: ClientError.ClientCooked,
        }
    }
}

export async function create_account(username: string, password: string): Promise<CreateUserAccountResponse> {
    const request:CreateUserAccountRequest  = {
        username: username,
        password: password,
    };
    try {
        const response = await fetch (`http://${SERVER_IP}:${SERVER_PORT}/users/api/create_account`,
               {
                    "method": "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(request)
       });
       if (!response.ok) {
           let response: CreateUserAccountResponse = {
               status: CreateUserAccountStatus.ServerError,
           };
           return response;
       }
       return response.json() as Promise<CreateUserAccountResponse>;
    } catch (error) {
        return {
            status: CreateUserAccountStatus.ServerError,
        };
    }
}
