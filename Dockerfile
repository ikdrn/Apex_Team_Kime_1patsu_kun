# ============================================================
# Dockerfile - チーム決め一発くん マルチステージビルド（修正版 v2）
#
# 【ビルドの流れ】
# Stage 1 (frontend-builder) : Node.js で Vue.js をビルド → dist/ を生成
# Stage 2 (backend-builder)  : Rust で バイナリをビルド
# Stage 3 (runtime)          : Alpine に バイナリ + dist/ だけを入れた軽量イメージ
#
# 【Renderデプロイ時の重要注意事項】
#   - Renderは起動時に PORT 環境変数を自動セットする
#   - RustコードはこのPORT変数を読み取ってリッスンする
#   - EXPOSE はドキュメント的な意味しかなくポートを固定しない
# ============================================================


# ──────────────────────────────────────────────────────────────
# Stage 1: フロントエンド（Vue.js）のビルド
# ──────────────────────────────────────────────────────────────
FROM node:20-alpine AS frontend-builder

WORKDIR /app/frontend

# ── 依存関係だけ先にインストール（Docker キャッシュ活用）──
# package*.json = package.json と package-lock.json の両方にマッチ
# ソースコードが変わっても package.json が変わらなければこの層はキャッシュされる
COPY frontend/package*.json ./
RUN npm ci --silent

# ── Vueアプリのビルド ──
COPY frontend/ .
RUN npm run build
# ビルド後は dist/ ディレクトリに HTML/JS/CSS が生成される


# ──────────────────────────────────────────────────────────────
# Stage 2: バックエンド（Rust）のビルド
#
# rust:1.81-alpine を使用する理由:
#   Alpine は musl libc を使うため、ビルドされたバイナリは
#   glibc に依存しない（= 軽量な Alpine の runtime でも動く）
# ──────────────────────────────────────────────────────────────
FROM rust:1.81-alpine AS backend-builder

# musl-dev: Alpine で C コードのコンパイルに必要なヘッダーファイル
# Rust クレートの一部（ring, hyper など）は C ライブラリを内部で使うため必要
RUN apk add --no-cache musl-dev

WORKDIR /app

# ────────────────────────────────────────────────────────────
# 【Dockerキャッシュ最適化テクニック】
#
# Cargo.toml だけ先にコピーし、ダミーの main.rs でビルドを実行する。
# こうすると:
#   - 依存クレートのダウンロード＆コンパイルはこのレイヤーに閉じ込められる
#   - 次回以降、Cargo.toml が変わらなければこのレイヤーはキャッシュから使われる
#   - ソースコード(src/)だけ変わった場合、依存のコンパイルをスキップできる
#
# 【注意】
#   Cargo.lock はコミットされていない場合があるため
#   Cargo.toml のみをコピーする（cargo が自動で Cargo.lock を生成する）
# ────────────────────────────────────────────────────────────
COPY backend/Cargo.toml ./

# ダミービルドで依存クレートをコンパイル＆キャッシュ
RUN mkdir -p src && \
    echo 'fn main() { println!("placeholder"); }' > src/main.rs && \
    cargo build --release && \
    # ダミービルドの成果物を削除する
    # （Rustはファイル変更日時でリビルド判定するため、古い成果物を消す必要がある）
    # apex-team-balancer の hyphens は deps では underscores になる
    rm -f target/release/deps/apex_team_balancer* && \
    rm -f target/release/apex-team-balancer && \
    rm -rf src

# ── 実際のソースコードをコピーしてビルド ──
COPY backend/src ./src
RUN cargo build --release

# バイナリが正しく生成されたか確認（デバッグ用: ビルドログに表示される）
RUN ls -lh target/release/apex-team-balancer


# ──────────────────────────────────────────────────────────────
# Stage 3: 本番実行環境（最小構成）
#
# Alpine Linux のみ。Rustコンパイラも Node.js も含まない。
# これにより最終イメージを数MB〜十数MBに抑えられる。
# ──────────────────────────────────────────────────────────────
FROM alpine:3.20 AS runtime

# ca-certificates: HTTPS通信での証明書検証に必要
# （外部APIを呼ぶ場合や、Render のヘルスチェック通信に必要）
RUN apk add --no-cache ca-certificates

# ── セキュリティ: 非rootユーザーで実行 ──
# root で実行すると脆弱性悪用時のリスクが高まるため、専用ユーザーを作る
RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

# 作業ディレクトリ = バイナリの起動ディレクトリ
# Rust コードの ServeDir::new("static") は
# このディレクトリを起点とした相対パスになる
WORKDIR /app

# ── ビルド成果物のコピー ──

# [1] Rustバイナリ（Stage 2 から）
COPY --from=backend-builder \
    /app/target/release/apex-team-balancer \
    ./apex-team-balancer

# [2] Vueのビルド成果物（Stage 1 から）
#     dist/ を static/ にコピーする。
#     Rustコードは ServeDir::new("static") でここを参照している。
COPY --from=frontend-builder \
    /app/frontend/dist \
    ./static

# 実行権限の付与
RUN chmod +x ./apex-team-balancer

# ファイルオーナーを非rootユーザーに変更
RUN chown -R appuser:appgroup /app

# 非rootユーザーに切り替え
USER appuser

# ────────────────────────────────────────────────────────────
# EXPOSE について
#
# EXPOSE はドキュメントとしての宣言で、実際のポート開放は行わない。
# Render は PORT 環境変数でポートを動的に割り当てるため、
# 固定ポートは記載しない。Rustコードが $PORT を読み取って適切にバインドする。
# ────────────────────────────────────────────────────────────
EXPOSE 8080

# ────────────────────────────────────────────────────────────
# ヘルスチェック
#
# Render の外部ヘルスチェック（render.yaml の healthCheckPath）とは別に、
# Dockerレベルでもコンテナの健全性を監視する。
# start-period=60s: 起動直後の猶予期間（Rustアプリの起動待ち）
# ────────────────────────────────────────────────────────────
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD wget -qO- "http://localhost:${PORT:-8080}/health" > /dev/null || exit 1

# ── 起動コマンド ──
CMD ["./apex-team-balancer"]
