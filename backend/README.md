## 命名規則

| アイテム                 | 規則                                      |
| ------------------------ | ----------------------------------------- |
| クレート                 | snake_case (単一の単語が好ましい)         |
| モジュール               | snake_case                                |
| 型                       | CamelCase                                 |
| トレイト                 | CamelCase                                 |
| Enumのバリアント         | CamelCase                                 |
| 関数                     | snake_case                                |
| メソッド                 | snake_case                                |
| 一般のコンストラクタ     | new または with_more_details              |
| 変換を行うコンストラクタ | from_some_other_type                      |
| マクロ                   | snake_case!                               |
| ローカル変数             | snake_case                                |
| スタティック変数         | SCREAMING_SNAKE_CASE                      |
| 定数                     | SCREAMING_SNAKE_CASE                      |
| 型パラメータ             | 簡潔なCamelCase、大抵は大文字で1文字: T   |
| ライフタイム             | 短いlowercase、大抵は1文字: 'a, 'de, 'src |


## 環境構築(docker を使用しない場合)
[公式](https://rustup.rs/)に従ってインストールしてください
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

この時点でcargoが使用できるようになります。
backendを実行するには、以下のコマンドを実行してください。
```bash
cargo run
```