import { useEffect, useState } from "react";

import { create_get } from "../utils/api"
import { SERVER_IP, SERVER_PORT } from "../types/types"
import { UserDetailsPublic, UserSessionStatus} from "../types/types"

import styles from "../styles/UserPage.module.css";

interface UserPageLayoutProps {
    username: string,
}

const UserPageLayout = (props: UserPageLayoutProps) => {
    return (
      <div className={ styles.parent }>
        <div className={ styles.side_bar }>
            <h3 id={ styles.side_bar_username }>{ props.username }</h3>
            <div className={ styles.side_bar_contents }>
                <button className={ styles.side_bar_button } >Click Me</button>
                <button className={ styles.side_bar_button } >Click Me</button>
                <button className={ styles.side_bar_button } >Click Me</button>
                <button className={ styles.side_bar_button } >Click Me</button>
                <button className={ styles.side_bar_button } >Click Me</button>
                <button className={ styles.side_bar_button } >Click Me</button>
            </div>
        </div>
        <div className={ styles.right }>
            <div className={ styles.navigation_bar }>
                <button className={ styles.nav_bar_button } >Click Me</button>
                <button className={ styles.nav_bar_button } >Click Me</button>
                <button className={ styles.nav_bar_button } >Click Me</button>
                <button className={ styles.nav_bar_button } >Click Me</button>
                <button className={ styles.nav_bar_button } >Click Me</button>
                <button className={ styles.nav_bar_button } >Click Me</button>
            </div>
            <div className={ styles.main_page }>
                <h2>Page Content Goes here...</h2>
            </div>
        </div>
       </div>
    );
}

const UserPage = () => {
    const [userName, setUserName] = useState("John Doe");
    useEffect(() => {
        // Add Body CSS.
        
        document.body.classList.add(styles.UserPageBody);


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

        // Return removed body
        return () => {
            document.body.classList.remove(styles.UserPageBody);
        };
    }, []);

    let userProps: UserPageLayoutProps = {
        username: userName,
    };

    return <UserPageLayout username={userName}/>;
}

export default UserPage;
