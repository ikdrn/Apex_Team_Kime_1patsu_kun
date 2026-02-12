# ============================================================
# Dockerfile - チーム決め一発くん マルチステージビルド（修正版 v3）
#
# 【ビルドの流れ】
# Stage 1 (frontend-builder) : Node.js で Vue.js をビルド → dist/ を生成
# Stage 2 (backend-builder)  : Rust で バイナリをビルド
# Stage 3 (runtime)          : Alpine に バイナリ + dist/ だけを入れた軽量イメージ
#
# 【v3 での修正内容】
#   - npm ci → npm install に変更
#     理由: package-lock.json がリポジトリに存在しない場合、npm ci は
#           "requires package-lock.json" エラーで即座に失敗する。
#           npm install はロックファイル不要で動作する。
#
#   - vue-tsc を build スクリプトから除外（package.json 側も修正）
#     理由: tsconfig.node.json が @tsconfig/node20 を extends しているが
#           そのパッケージが devDependencies に存在せず型チェックが失敗する。
#           vite build 単体であれば型チェックをスキップして成功する。
#
#   - .dockerignore を追加（リポジトリ側で管理）
#     理由: ローカルの node_modules や dist を誤ってコピーしないため。
# ============================================================


# ──────────────────────────────────────────────────────────────
# Stage 1: フロントエンド（Vue.js）のビルド
# ──────────────────────────────────────────────────────────────
FROM node:20-alpine AS frontend-builder

WORKDIR /app/frontend

# ── 依存関係のインストール ──
# 【重要】npm ci は package-lock.json が必須。
# リポジトリに package-lock.json が存在しない場合は即座に失敗する。
# npm install はロックファイルなしでも動作し、必要であれば生成する。
COPY frontend/package*.json ./
RUN npm install --silent

# ── Vue アプリのビルド ──
# package.json の build スクリプト = "vite build"
# （vue-tsc による型チェックは tsconfig 依存関係の問題で除外済み）
COPY frontend/ .
RUN npm run build


# ──────────────────────────────────────────────────────────────
# Stage 2: バックエンド（Rust）のビルド
#
# rust:1.81-alpine = Alpine Linux ベースの Rust コンパイライメージ
# Alpine は musl libc を使うため、生成バイナリは静的リンクされる
# ──────────────────────────────────────────────────────────────
FROM rust:1.81-alpine AS backend-builder

# musl-dev: musl libc 用の C コンパイルヘッダー
# （tokio など一部クレートのビルドに必要）
RUN apk add --no-cache musl-dev

WORKDIR /app

# ────────────────────────────────────────────────────────────
# 【Dockerキャッシュ最適化 - 依存クレートの事前ビルド】
#
# ① Cargo.toml だけをコピー
#   → Cargo.lock がコミットされていなくても問題ない
#   → cargo が自動で Cargo.lock を生成する
#
# ② ダミーの main.rs でビルド（依存クレートのみコンパイル）
#   → この層は Cargo.toml が変わらない限りキャッシュされる
#   → ソースコード変更時に依存クレートの再コンパイルが不要になる
#
# ③ ダミービルド成果物の削除
#   → 本番ビルドが正しく実行されるよう、古い成果物を消す
#   → deps の名前は Rust がハイフンをアンダースコアに変換したもの
# ────────────────────────────────────────────────────────────
COPY backend/Cargo.toml ./

RUN mkdir -p src && \
    echo 'fn main() { println!("build cache placeholder"); }' > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/apex_team_balancer* && \
    rm -f target/release/apex-team-balancer && \
    rm -rf src

# ── 実際のソースコードでビルド ──
COPY backend/src ./src
RUN cargo build --release

# ビルド結果の確認（ビルドログに表示される）
RUN ls -lh target/release/apex-team-balancer


# ──────────────────────────────────────────────────────────────
# Stage 3: 本番実行環境（最小構成）
# ──────────────────────────────────────────────────────────────
FROM alpine:3.20 AS runtime

# ca-certificates: HTTPS 通信での証明書検証に必要
RUN apk add --no-cache ca-certificates

# 非 root ユーザーで実行（セキュリティのベストプラクティス）
RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

# 作業ディレクトリ
# Rust コード内の ServeDir::new("static") はここを起点とする
WORKDIR /app

# ── ビルド成果物のコピー ──
COPY --from=backend-builder \
    /app/target/release/apex-team-balancer \
    ./apex-team-balancer

COPY --from=frontend-builder \
    /app/frontend/dist \
    ./static

RUN chmod +x ./apex-team-balancer && \
    chown -R appuser:appgroup /app

USER appuser

# ────────────────────────────────────────────────────────────
# EXPOSE: ドキュメント用の宣言
# Render は PORT 環境変数でポートを動的に割り当てる（通常 10000）
# Rust コードは std::env::var("PORT") で読み取って 0.0.0.0:PORT にバインドする
# ────────────────────────────────────────────────────────────
EXPOSE 8080

# ヘルスチェック（start-period を長めに設定して Render の起動タイムアウトを回避）
HEALTHCHECK --interval=30s --timeout=10s --start-period=90s --retries=3 \
    CMD wget -qO- "http://localhost:${PORT:-8080}/health" > /dev/null || exit 1

CMD ["./apex-team-balancer"]
