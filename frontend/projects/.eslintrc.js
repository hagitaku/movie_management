module.exports = {
  extends: [
    "next/core-web-vitals",
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "plugin:react/recommended",
    "plugin:react-hooks/recommended",
    "plugin:react/jsx-runtime",
    "plugin:eslint-comments/recommended",
    "plugin:storybook/recommended",
    "prettier",
  ],
  env: { browser: true, node: true, es6: true },
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaVersion: "latest",
    sourceType: "module",
    project: "./tsconfig.json",
    tsconfigRootDir: __dirname,
  },
  plugins: ["@typescript-eslint", "simple-import-sort", "import"],
  ignorePatterns: ["node_modules/", ".eslintrc.js"],
  rules: {
    "react/jsx-curly-brace-presence": "warn", // 不要な波括弧禁止
    "simple-import-sort/imports": "error", // importとexportソート
    "simple-import-sort/exports": "error",
    "import/first": "error", // importを先頭に
    "import/newline-after-import": "error", // importの後に改行
    "import/no-duplicates": "error", // importの重複省く
    "@typescript-eslint/no-explicit-any": "error", // any禁止
    "@typescript-eslint/no-unused-vars": "error", // 未使用の変数禁止
    "react/jsx-pascal-case": "error", // パスカルケースでコンポーネントを定義
    "object-shorthand": ["warn", "properties", { avoidQuotes: true }],
    "eslint-comments/require-description": "error", //eslint-disable-next-lineのコメントは必ず説明を書く。https://mysticatea.github.io/eslint-plugin-eslint-comments/rules/require-description.html
    "eslint-comments/disable-enable-pair": ["error", { allowWholeFile: true }], //https://mysticatea.github.io/eslint-plugin-eslint-comments/rules/disable-enable-pair.html
    "react/function-component-definition": [
      "error", // 関数コンポーネントはアロー関数で定義
      { namedComponents: "arrow-function" },
    ],
    "no-magic-numbers": [
      "error", // マジックナンバーの使用禁止．ただし，以下は省く．
      {
        ignore: [-1, 0, 1], //配列検索でindexOf === -1などは許容する
        ignoreDefaultValues: true, //const { tax = 0.1 } = props
        ignoreArrayIndexes: true, //data[100] ok
        enforceConst: true, //マジックナンバーはconstで定義する
      },
    ],
  },
  overrides: [
    // Next.jsのファイルルーティングはexport defaultが必要
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
      rules: { "no-magic-numbers": "off" }, // constファイルでは許容する
    },
  ],
};
