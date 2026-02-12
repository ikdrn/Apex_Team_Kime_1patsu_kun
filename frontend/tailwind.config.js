// ============================================================
// tailwind.config.js - Tailwind CSSの設定ファイル
//
// 【デザイン方針】
// - 禁止: 暗い背景に紫グラデーション
// - 採用: 白/黒/グレー + アクセントカラー（青 + オレンジ）
// - スタイル: 建築的・エディトリアル（余白重視、グリッドベース）
// ============================================================

/** @type {import('tailwindcss').Config} */
export default {
  // Tailwindを適用するファイルのパターン
  // これ以外のファイルには不要なCSSクラスは生成されない（ファイルサイズ削減）
  content: [
    './index.html',
    './src/**/*.{vue,js,ts,jsx,tsx}',
  ],

  theme: {
    extend: {
      // ── カスタムカラーパレット ──
      colors: {
        // プライマリカラー: ディープブルー（アクションボタン、強調に使用）
        primary: {
          50:  '#eff6ff',
          100: '#dbeafe',
          200: '#bfdbfe',
          300: '#93c5fd',
          400: '#60a5fa',
          500: '#3b82f6',
          600: '#2563eb',
          700: '#1d4ed8', // メインのプライマリカラー
          800: '#1e40af',
          900: '#1e3a8a',
        },
        // アクセントカラー: バーントオレンジ（危険・警告・ハイライトに使用）
        accent: {
          50:  '#fff7ed',
          100: '#ffedd5',
          200: '#fed7aa',
          300: '#fdba74',
          400: '#fb923c',
          500: '#f97316',
          600: '#ea580c', // メインのアクセントカラー
          700: '#c2410c',
          800: '#9a3412',
          900: '#7c2d12',
        },
        // ニュートラル: ほぼグレースケール（背景・テキスト・ボーダーに使用）
        neutral: {
          0:   '#ffffff',
          50:  '#fafafa',
          100: '#f5f5f5',
          200: '#e5e5e5',
          300: '#d4d4d4',
          400: '#a3a3a3',
          500: '#737373',
          600: '#525252',
          700: '#404040',
          800: '#262626',
          900: '#171717',
          950: '#0a0a0a',
        },
      },

      // ── タイポグラフィ ──
      fontFamily: {
        // システムフォントスタック（読み込み不要で即座に表示）
        sans: [
          '"Noto Sans JP"',
          'ui-sans-serif',
          'system-ui',
          '-apple-system',
          'BlinkMacSystemFont',
          '"Segoe UI"',
          'Roboto',
          'sans-serif',
        ],
        // 見出し用: よりシャープなフォントスタック
        display: [
          '"Noto Sans JP"',
          'ui-sans-serif',
          'system-ui',
          'sans-serif',
        ],
        // 等幅フォント（コードやID表示に使用）
        mono: [
          'ui-monospace',
          'SFMono-Regular',
          '"SF Mono"',
          'Consolas',
          'monospace',
        ],
      },

      // ── カスタムスペーシング ──
      spacing: {
        '18': '4.5rem',
        '88': '22rem',
        '120': '30rem',
      },

      // ── ボーダー半径 ──
      // エディトリアルデザインではあまり丸くしない
      borderRadius: {
        'sm': '0.125rem', // ほぼシャープ
        DEFAULT: '0.25rem',
        'md': '0.375rem',
        'lg': '0.5rem',
      },

      // ── アニメーション ──
      keyframes: {
        'slide-up': {
          '0%': { transform: 'translateY(8px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        'fade-in': {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
      },
      animation: {
        'slide-up': 'slide-up 0.3s ease-out',
        'fade-in': 'fade-in 0.2s ease-out',
      },
    },
  },

  plugins: [],
}
