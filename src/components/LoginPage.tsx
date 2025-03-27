import { useState } from "react"

import { helloWorld } from "../utils/api"

import "../styles/login.css"

function LoginPage() {
    const [title, setTitle] = useState("CO-LOGIN");


   function loginCallback(e) {
        e.preventDefault();

        const form = e.target;
        const formData = new FormData(form);

        console.log(typeof(e));
        const formJson = Object.fromEntries(formData.entries());
        console.log(formJson.username);
        console.log(formJson.password);
  }



    return (
        <>
            <div className="login_div">
            <form onSubmit={ loginCallback }>
                <h1 id="co_heading">{ title }</h1>
                <label id="username_label" className="label_common" htmlFor="username"> Username: 
                    <input name="username" type="text" id="username_input" className="input_common"></input> <br />
                </label>
                <label id="password_label" className="label_common" htmlFor="password"> Password: 
                    <input name="password" type="password" id="password_input" className="input_common"></input> <br />
                </label>
                <button type="submit" id="login_button" className="button_common">Login</button>
                <button id="create_account_button" className="button_common">New Account</button>
            </form>
            </div>
        </>
    );
}

export default LoginPage;
