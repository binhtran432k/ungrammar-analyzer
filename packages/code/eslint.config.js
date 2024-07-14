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
  {
    files: ["**/test/suite/*.ts"],
    rules: {
      "@typescript-eslint/no-unsafe-call": "off",
      "@typescript-eslint/no-unsafe-member-access": "off",
    },
  },
  eslintConfigPrettier,
  {
    ignores: [
      "**/dist/",
      "**/.vscode-test/",
      "**/.vscode-test-web/",
      "esbuild.js",
      "eslint.config.js",
    ],
  },
];
