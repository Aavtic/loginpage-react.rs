import { SERVER_IP, SERVER_PORT } from "../types/types";

export async function helloWorld(): Promise<String> {
        try {
          const res = await fetch(`http://${SERVER_IP}:${SERVER_PORT}/users/api/hello_world`, {
            method: "GET",
          });

          if (!res.ok) {
            throw new Error(`HTTP error! Status: ${res.status}`);
          }

          return res.text();
        } catch (error) {
            throw error;
        }
}
