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
  globalIgnores(["**/node_modules/", "*.config.js",".eslintrc.js"]),
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
      "react/jsx-curly-brace-presence": "warn", // 不要な波括弧禁止
      "simple-import-sort/imports": "error", // importとexportのソート
      "simple-import-sort/exports": "error",
      "import/first": "error", // importを先頭に
      "import/newline-after-import": "error", // importの後に改行
      "import/no-duplicates": "error", // importの重複禁止
      "@typescript-eslint/no-explicit-any": "error", // any禁止
      "@typescript-eslint/no-unused-vars": "error", // 未使用変数の禁止
      "react/jsx-pascal-case": "error", // パスカルケースでコンポーネントを定義

      "object-shorthand": [
        "warn",
        "properties",
        {
          avoidQuotes: true,
        },
      ],

      "eslint-comments/require-description": "error", //eslint-disable-next-lineのコメントは必ず説明を書く。https://mysticatea.github.io/eslint-plugin-eslint-comments/rules/require-description.html

      "eslint-comments/disable-enable-pair": [
        "error", //https://mysticatea.github.io/eslint-plugin-eslint-comments/rules/disable-enable-pair.html
        {
          allowWholeFile: true,
        },
      ],

      "react/function-component-definition": [
        "error", // 関数コンポーネントはアロー関数で定義
        {
          namedComponents: "arrow-function",
        },
      ],

      "no-magic-numbers": [
        "error", // マジックナンバーの使用禁止．ただし，以下は除く．
        {
          ignore: [-1, 0, 1], //配列検索でindexOf === -1などは許容する
          ignoreDefaultValues: true, //const { tax = 0.1 } = props
          ignoreArrayIndexes: true, //data[100] ok
          enforceConst: true, //マジックナンバーはconstで定義する
        },
      ],
    },
  },
  {
    // Next.jsのファイルルーティングはexport defaultが必要
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
      "no-magic-numbers": "off", // constファイルでは許容する
    },
  },
]);
