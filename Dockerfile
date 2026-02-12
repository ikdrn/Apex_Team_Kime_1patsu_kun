# ============================================================
# Dockerfile - チーム決め一発くん マルチステージビルド定義
#
# 【マルチステージビルドとは】
# 1つのDockerfileで複数のビルドフェーズを定義する技術。
# 最終イメージにはビルドツール（Node.js, Rustコンパイラ等）を
# 含めないため、デプロイイメージを軽量に保てる。
#
# 【ビルドの流れ】
# Stage 1 (frontend-builder) : Vue.js をビルドして dist/ を生成
# Stage 2 (backend-builder)  : Rust をビルドして実行ファイルを生成
# Stage 3 (runtime)          : 実行ファイル + dist/ だけを持つ軽量イメージ
# ============================================================


# ──────────────────────────────────────────────────────────────
# Stage 1: フロントエンド（Vue.js）のビルド
# Node.js 20 Alpine（Alpine = 軽量Linuxディストリビューション）を使用
# ──────────────────────────────────────────────────────────────
FROM node:20-alpine AS frontend-builder

# 作業ディレクトリを設定
# 以降の COPY, RUN コマンドはこのディレクトリ内で実行される
WORKDIR /app/frontend

# ── 依存関係のインストール ──
# package.json と package-lock.json を先にコピーしてから npm ci を実行する。
# こうすることで、ソースコードが変わっても依存関係が変わらない限り
# Docker のキャッシュが使われ、ビルド時間が短縮される。
COPY frontend/package*.json ./
RUN npm ci --silent

# ── ソースコードをコピーしてビルド ──
# npm run build → vite build を実行し dist/ ディレクトリにバンドルを出力する
COPY frontend/ .
RUN npm run build


# ──────────────────────────────────────────────────────────────
# Stage 2: バックエンド（Rust）のビルド
# rust:1.81-alpine を使用（Alpine で musl コンパイルにより静的リンク）
# ──────────────────────────────────────────────────────────────
FROM rust:1.81-alpine AS backend-builder

# musl-dev: musl libc のヘッダーファイル（Alpine でのコンパイルに必要）
# pkgconfig, openssl-dev: 一部クレートが SSL を必要とする場合に備えて
RUN apk add --no-cache musl-dev pkgconfig openssl-dev openssl-libs-static

WORKDIR /app

# ── 依存関係のキャッシュ最適化（重要なテクニック）──
# まず Cargo.toml と Cargo.lock だけをコピーし、
# ダミーの main.rs でビルドを実行する。
# こうすると、ソースコードのみが変わった場合に依存クレートの
# ダウンロード・コンパイルをスキップできる（大幅な時間短縮）。
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && \
    echo 'fn main() { println!("dummy"); }' > src/main.rs && \
    cargo build --release 2>&1 && \
    # ダミービルドの成果物を削除（実際のビルドと競合しないように）
    rm -f target/release/deps/apex_team_balancer* && \
    rm src/main.rs

# ── 実際のソースコードでビルド ──
COPY backend/src ./src
RUN cargo build --release


# ──────────────────────────────────────────────────────────────
# Stage 3: 実行環境（最小限の軽量イメージ）
# Alpine Linux のみ（Rustコンパイラも Node.js も含まない）
# ──────────────────────────────────────────────────────────────
FROM alpine:3.20 AS runtime

# ── セキュリティのためのランタイム依存関係 ──
# ca-certificates: HTTPS通信の証明書検証に必要
# libgcc: Rustの実行ファイルが動的リンクする基本ライブラリ
RUN apk add --no-cache ca-certificates libgcc

# 非rootユーザーで実行（セキュリティのベストプラクティス）
RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

WORKDIR /app

# ── ビルド成果物をコピー ──
# Rustの実行ファイル（Stage 2 から）
COPY --from=backend-builder /app/target/release/apex-team-balancer ./apex-team-balancer

# Vueのビルド成果物（Stage 1 から）
# Rustサーバーが ./static ディレクトリを静的ファイルルートとして使用する
COPY --from=frontend-builder /app/frontend/dist ./static

# 実行ファイルに実行権限を付与
RUN chmod +x ./apex-team-balancer

# ファイルの所有者を非rootユーザーに変更
RUN chown -R appuser:appgroup /app

# 非rootユーザーに切り替え
USER appuser

# ── ポートの公開 ──
# Renderは PORT 環境変数でポートを指定する（デフォルト: 8080）
EXPOSE 8080

# ── ヘルスチェック ──
# Dockerがコンテナの健全性を定期的に確認する設定
# /health エンドポイントが200を返せば「健全」と判断
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD wget -qO- http://localhost:${PORT:-8080}/health || exit 1

# ── 起動コマンド ──
CMD ["./apex-team-balancer"]
