import {defineConfig} from 'vite'
// import {fileURLToPath} from 'url'
import react from '@vitejs/plugin-react'
import path from 'path'

// const __filename = fileURLToPath(import.meta.url);
// const __dirname=path.dirname(__filename);

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    // alias: [{ find: "@", replacement: path.resolve(__dirname, "src") }],
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  }
})
