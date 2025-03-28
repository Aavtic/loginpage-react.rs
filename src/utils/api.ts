import { SERVER_IP, SERVER_PORT, ClientError, Result} from "../types/types";
import { CreateUserAccountRequest, CreateUserAccountResponse, CreateUserAccountStatus } from "../types/types";


export async function create_post<PostType, ExpectedType>(url: string, data: PostType): Promise<Result<ExpectedType>> {
    try {
        const response = await fetch (url,
               {
                    "method": "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(data)
       });
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
