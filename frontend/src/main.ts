// ============================================================
// main.ts - Vueアプリケーションのエントリーポイント
//
// 【このファイルの役割】
// 1. Vueアプリを作成する（createApp）
// 2. プラグインを登録する（Pinia, Vue Router）
// 3. index.html の <div id="app"> にアプリをマウントする
// ============================================================

// Tailwind CSSをインポート（全コンポーネントに適用される）
import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'

// ── Vueアプリの作成 ──
// App.vue がルートコンポーネントになる
const app = createApp(App)

// ── Piniaの登録 ──
// use() でプラグインを登録する。順番は重要（routerより先にPiniaを登録）
app.use(createPinia())

// ── Vue Routerの登録 ──
app.use(router)

// ── アプリのマウント ──
// index.html の <div id="app"> にVueをアタッチする
app.mount('#app')
