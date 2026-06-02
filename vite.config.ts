import { defineConfig } from "vite-plus";

export default defineConfig({
  lint: {
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: true, typeCheck: true },
  },
  staged: {
    "pnpm-lock.yaml": "cargo codegen",
    "*.{js,jsx,mjs,cjs,ts,tsx,mts,cts,json,jsonc,md,yml,toml}": "vp fmt",
  },
});
