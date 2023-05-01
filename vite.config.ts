import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import solidPlugin from 'vite-plugin-solid';

// https://vitejs.dev/config/
export default defineConfig({
  // plugins: [react(), solidPlugin()]
  plugins: [solidPlugin()]
})
