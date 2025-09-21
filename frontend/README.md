## 命名規則

| カテゴリ       | ケース           |
| -------------- | ---------------- |
| 型エイリアス   | PascalCase       |
| 変数,引数      | camelCase        |
| 関数,メソッド  | camelCase        |
| プロパティ     | camelCase        |
| コンポーネント | PascalCase       |
| 定数           | UPPER_SNAKE_CASE |

### 環境構築(docker を使わない場合)

[公式ページのインストール方法](https://nodejs.org/ja/download "Node.js install page") に従って npm パッケージ をインストールしてください(2025 年 9 月時点)

`frontend/projects/`内で以下のコマンドを実行してください．依存関係のインストールが行われます．

```
npm install
```

以下のコマンドを実行することで next プロジェクトが起動します．

```
npm run dev
```
