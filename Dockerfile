# Mosy の Rust 開発環境用 Dockerfile
FROM rust:1.68-slim

# Tauri および onnxruntime のビルドに必要な依存パッケージをインストール
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    libwebkit2gtk-4.0-dev \
    && rm -rf /var/lib/apt/lists/*

# フロントエンド開発用に Node.js LTS をインストール
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && apt-get install -y nodejs

# tauri-cli をインストール
RUN cargo install tauri-cli

WORKDIR /app
CMD ["cargo", "tauri", "dev"]
