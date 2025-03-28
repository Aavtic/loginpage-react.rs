import { useState } from "react"

import { create_post } from "../utils/api"
import { CreateUserAccountRequest, CreateUserAccountResponse, CreateUserAccountStatus } from "../types/types"
import { SERVER_IP, SERVER_PORT, ClientError} from "../types/types";

import "../styles/login.css"

function LoginPage() {
    const [title, setTitle] = useState("CO-LOGIN");
    async function loginCallback(e: React.FormEvent<HTMLFormElement>) {
        console.log("clicked...");
        e.preventDefault();

        const submitter = (e.nativeEvent as SubmitEvent).submitter as HTMLButtonElement;
        const action = submitter?.value;
        const formData = new FormData(e.currentTarget);
        const creds = Object.fromEntries(formData.entries());

        if (action === 'login') {
        } else if (action === 'create') {
            console.log('sending create acccout req');
            const username: string = creds.username.toString();
            const password: string = creds.password.toString();
            let req: CreateUserAccountRequest = {
                username,
                password,
            }
            let response = await create_post<CreateUserAccountRequest, CreateUserAccountResponse>(`http://${SERVER_IP}:${SERVER_PORT}/users/api/create_account`, req);
            if (!response.ok) {
                setTitle("Server or network seems to be dead...Hope it's the network");
            } else {
                console.log("status = " + response.data.status);
                if (response.data.status === CreateUserAccountStatus.ServerError) {
                    setTitle("Server or network seems to be dead...Hope it's the network");
                } else if (response.data.status === CreateUserAccountStatus.UsernameAlreadyExists) {
                    console.log("Changing...");
                    setTitle("Too slow, that user already exists lol");
                } else if (response.data.status === CreateUserAccountStatus.Success){
                    setTitle("Welcome, " + username);
                }
            }
        }
    }



    return (
        <>
            <div className="login_div">
                <form onSubmit={loginCallback}>
                    <h1 id="co_heading">{title}</h1>
                    <label id="username_label" className="label_common" htmlFor="username"> Username:
                        <input name="username" type="text" id="username_input" className="input_common"></input> <br />
                    </label>
                    <label id="password_label" className="label_common" htmlFor="password"> Password:
                        <input name="password" type="password" id="password_input" className="input_common"></input> <br />
                    </label>
                    <button type="submit" name="action" value="login" id="login_button" className="button_common">Login</button>
                    <button type="submit" name="action" value="create" id="create_account_button" className="button_common">New Account</button>
                </form>
            </div>
        </>
    );
}

export default LoginPage;
