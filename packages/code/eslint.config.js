const eslintConfigPrettier = require("eslint-config-prettier");
const js = require("@eslint/js");
const ts = require("typescript-eslint");

/** @type {import("eslint").Linter.FlatConfig[]} */
module.exports = [
  js.configs.recommended,
  ...ts.configs.recommendedTypeChecked,
  {
    languageOptions: {
      parserOptions: {
        project: true,
      },
    },
  },
  eslintConfigPrettier,
  {
    ignores: ["**/out/", "**/dist/", "**/.vscode-test/", "esbuild.js", "eslint.config.js"],
  },
];
