import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

import LoginPage from "./components/LoginPage"
import UserPage from "./components/UserPage"

import {RouterProvider, createBrowserRouter} from 'react-router-dom'


const router = createBrowserRouter([
    {
        path: "/login",
        element: <LoginPage />
    },

    {
        path: "/user",
        element: <UserPage />
    },
]);

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <RouterProvider router={router} />
  </StrictMode>,
)
