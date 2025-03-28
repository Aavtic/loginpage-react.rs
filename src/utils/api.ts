import { SERVER_IP, SERVER_PORT } from "../types/types";
import { CreateUserAccountRequest } from "../types/types";

export async function create_account(username: string, password: string) {
    const request:CreateUserAccountRequest  = {
        username: username,
        password: password,
    };
    try {
        fetch (`http://${SERVER_IP}:${SERVER_PORT}/users/api/create_account`,
               {
                    "method": "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(request)
               }
        ).then((response) => {
            if (!response.ok) {
                console.log("response: " + response);
            }
            console.log("succeess " + response);
        });
    } catch (error) {
        console.log(error);
    }
}
