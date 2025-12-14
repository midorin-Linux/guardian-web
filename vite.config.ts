import path from "path";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import { visualizer } from "rollup-plugin-visualizer";

// https://vite.dev/config/
export default defineConfig({
    plugins: [
        react(),
        tailwindcss(),
        visualizer({
            open: true, // Automatically open the report in your browser
            filename: "dist/stats.html", // Output file
        }),
    ],
    build: {
        outDir: "static",
        emptyOutDir: true,
    },
    resolve: {
        alias: {
            "@": path.resolve(__dirname, "./ts-src"),
        },
    },
});
