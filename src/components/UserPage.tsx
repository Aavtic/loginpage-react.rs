import { useParams } from "react-router"
import { useEffect, useState } from "react";

import { create_get } from "../utils/api"
import { SERVER_IP, SERVER_PORT } from "../types/types"
import { Result } from "../types/types"
import { UserDetailsPublic, UserSessionStatus} from "../types/types"

const userPage = () => {
    const [userName, setUserName] = useState("John Doe");
    useEffect(() => {
        let cookie = document.cookie;
        if (!cookie) {
            window.location.href = "/login";
        }
        let key = cookie.split("=")[1];

        const getResult = async () => {
            let result = await create_get<UserDetailsPublic> (`http://${SERVER_IP}:${SERVER_PORT}/users/api/validate_session/${key}`, true);
            console.log("result: ", result);
            if (!result.ok) {
                console.log("Oops...");
                return;
            }
            const publicData = result.data;
            console.log("public data", publicData);
            if (publicData.status == UserSessionStatus.Ok) {
                setUserName(publicData.username.toString());
                console.log("Logged In!");
            } else if (publicData.status == UserSessionStatus.LoginRequired) {
                console.log("Login required.");
                window.location.href = "/login";
            }
        }
        getResult().catch(console.error);
    }, []);
    return <h1>Welcome, { userName }!</h1>
}

export default userPage;
