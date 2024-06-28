# ベースイメージとしてRustの公式イメージを使用
FROM rust:latest

# ワーキングディレクトリを作成
WORKDIR /usr/src/app

# Cargo.tomlとCargo.lockをコピー
COPY Cargo.toml Cargo.lock ./

# 依存関係をダウンロード
RUN cargo build --release

# ソースコードをコピー
COPY . .

# アプリケーションをビルド
RUN cargo build --release

# 実行可能ファイルを実行
CMD ["./target/release/trend"]