# salvo playground

Rust のフレームワークの salvo の検証用リポジトリ

## 構成情報

- Rust: 1.80.0
- Salvo: 0.68.5

## ローカル環境構築

**パッケージのインストールする**

```sh
cargo install cargo-watch
```

**ライブリロード用のライブラリをインストールする**

```sh
cargo install --path .
```

**salvo を起動する**

```sh
cargo watch -x run

# Log Level を DEBUG で起動する場合
RUST_LOG=DEBUG cargo watch -x run
```

### 動作確認

```sh
curl http://localhost:5800
```

## UT

**テストを実行する**

```sh
# すべてのテストを実行する
cargo test

# テスト対象のファイルを指定し実行する
# cargo test --test <ファイル名>
cargo test --test index

# テスト対象の関数を指定し実行する
# cargo test --test <ファイル名> <関数名>
cargo test --test users test_get_users
```

## ドキュメント

- [Home | Salvo](https://salvo.rs)
- [salvo-rs/salvo: A powerful web framework built with a simplified design.](https://github.com/salvo-rs/salvo)
