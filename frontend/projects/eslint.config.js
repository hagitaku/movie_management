import { defineConfig, globalIgnores } from "eslint/config";
import { fixupConfigRules, fixupPluginRules } from "@eslint/compat";
import typescriptEslint from "@typescript-eslint/eslint-plugin";
import simpleImportSort from "eslint-plugin-simple-import-sort";
import _import from "eslint-plugin-import";
import globals from "globals";
import tsParser from "@typescript-eslint/parser";
import path from "node:path";
import { fileURLToPath } from "node:url";
import js from "@eslint/js";
import { FlatCompat } from "@eslint/eslintrc";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const compat = new FlatCompat({
  baseDirectory: __dirname,
  recommendedConfig: js.configs.recommended,
  allConfig: js.configs.all,
});

export default defineConfig([
  globalIgnores(["**/node_modules/", "**/.eslintrc.js","/*.js"]),
  {
    extends: fixupConfigRules(
      compat.extends(
        "next/core-web-vitals",
        "eslint:recommended",
        "plugin:@typescript-eslint/recommended",
        "plugin:react/recommended",
        "plugin:react-hooks/recommended",
        "plugin:react/jsx-runtime",
        "plugin:eslint-comments/recommended",
        "prettier"
      )
    ),
    plugins: {
      "@typescript-eslint": fixupPluginRules(typescriptEslint),
      "simple-import-sort": simpleImportSort,
      import: fixupPluginRules(_import),
    },

    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },

      parser: tsParser,
      ecmaVersion: "latest",
      sourceType: "module",

      parserOptions: {
        project: "./tsconfig.json",
        tsconfigRootDir:
          "/Users/hokto/Desktop/movie_management-develop/frontend/projects",
      },
    },

    rules: {
      "react/jsx-curly-brace-presence": "warn",
      "simple-import-sort/imports": "error",
      "simple-import-sort/exports": "error",
      "import/first": "error",
      "import/newline-after-import": "error",
      "import/no-duplicates": "error",
      "@typescript-eslint/no-explicit-any": "error",
      "@typescript-eslint/no-unused-vars": "error",
      "react/jsx-pascal-case": "error",

      "object-shorthand": [
        "warn",
        "properties",
        {
          avoidQuotes: true,
        },
      ],

      "eslint-comments/require-description": "error",

      "eslint-comments/disable-enable-pair": [
        "error",
        {
          allowWholeFile: true,
        },
      ],

      "react/function-component-definition": [
        "error",
        {
          namedComponents: "arrow-function",
        },
      ],

      "no-magic-numbers": [
        "error",
        {
          ignore: [-1, 0, 1],
          ignoreDefaultValues: true,
          ignoreArrayIndexes: true,
          enforceConst: true,
        },
      ],
    },
  },
  {
    files: ["*/pages/**/**.tsx"],

    rules: {
      "import/no-default-export": "off",
      "import/prefer-default-export": "error",
      "@typescript-eslint/naming-convention": "off",
    },
  },
  {
    files: ["*/**/const/**/**.ts"],

    rules: {
      "no-magic-numbers": "off",
    },
  },
]);
