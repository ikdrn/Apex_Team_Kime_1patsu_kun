// ============================================================
// vite.config.ts - Vite（フロントエンドビルドツール）の設定ファイル
//
// 【このファイルの役割】
// 1. Vue.jsプラグインを有効化する
// 2. 開発サーバーのプロキシ設定（Rustバックエンドへ転送）
// 3. ビルド設定（出力ディレクトリの指定など）
// ============================================================
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { resolve } from 'path'

export default defineConfig({
  // ── プラグイン設定 ──
  // @vitejs/plugin-vue を使ってVueの単一ファイルコンポーネント(.vue)をコンパイルする
  plugins: [vue()],

  // ── パスエイリアス設定 ──
  // import文で '@/' が 'src/' を意味するように設定
  // 例: import { Player } from '@/types' → src/types/index.ts を参照
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
    },
  },

  // ── 開発サーバー設定 ──
  server: {
    port: 5173, // Viteのデフォルト開発ポート
    // プロキシ設定: /api/* へのリクエストをRustバックエンドに転送する
    // これにより開発中も本番と同じURLでAPIを呼べる
    proxy: {
      '/api': {
        target: 'http://localhost:8080', // Rustサーバーのアドレス
        changeOrigin: true,
        // リトライなどは不要（シンプルなプロキシ）
      },
    },
  },

  // ── ビルド設定 ──
  build: {
    // 出力先ディレクトリ（Dockerfileでここをコピーする）
    outDir: 'dist',
    // ソースマップは本番では不要なので無効化（バンドルサイズ削減）
    sourcemap: false,
    // チャンク分割の設定（パフォーマンス最適化）
    rollupOptions: {
      output: {
        // ライブラリ（vue, vue-router等）を別チャンクとして分割
        // ブラウザキャッシュの効率を上げるため
        manualChunks: {
          vendor: ['vue', 'vue-router', 'pinia'],
        },
      },
    },
  },
})
