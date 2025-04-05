import { useParams } from "react-router"
import create_get from "../utils/api"
import { useEffect } from "react";

const userPage = () => {
    let params = useParams();
    return <h1>Hello { params.username }!</h1>
}

export default userPage;
