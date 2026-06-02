import { defineConfig } from "vite-plus";

export default defineConfig({
  lint: {
    jsPlugins: [{ name: "vite-plus", specifier: "vite-plus/oxlint-plugin" }],
    rules: { "vite-plus/prefer-vite-plus-imports": "error" },
    options: { typeAware: true, typeCheck: true },
  },
  fmt: {
    ignorePatterns: ["CHANGELOG.md", "pnpm-lock.yaml", "pnpm-workspace.yaml"],
  },
  staged: {
    "pnpm-lock.yaml": "cargo codegen",
    "*.{js,jsx,mjs,cjs,ts,tsx,mts,cts,json,jsonc,md,yml,toml}": "vp fmt",
  },
});
