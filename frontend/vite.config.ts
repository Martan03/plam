import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import wasm from "vite-plugin-wasm";

// https://vite.dev/config/
export default defineConfig({
    plugins: [svelte(), wasm()],
    server: { fs: { allow: [".."] } },
    base: process.env.NODE_ENV === "production" ? "/plam/" : "/",
});
