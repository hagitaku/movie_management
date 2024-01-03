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