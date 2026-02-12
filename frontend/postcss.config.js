// postcss.config.js - PostCSS設定
// TailwindCSSとAutoprefixerを有効化する
// Tailwind CSS はPostCSSプラグインとして動作するため、この設定が必要
export default {
  plugins: {
    tailwindcss: {},    // Tailwind CSSユーティリティクラスを生成
    autoprefixer: {},   // ベンダープレフィックス（-webkit-等）を自動付与
  },
}
