import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  server: {
    host: "localhost", // Ensure it's bound to localhost
    port: 5173,        // Default Vite port
    https: false       // Force HTTP instead of HTTPS
  }
});
