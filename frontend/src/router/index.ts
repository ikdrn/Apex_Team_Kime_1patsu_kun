// ============================================================
// router/index.ts - Vue Routerの設定
//
// 【Vue Routerとは】
// URLのパスに応じて表示するコンポーネントを切り替えるライブラリ。
// / → UserView（一般ユーザー用画面）
// /owner → OwnerView（管理者用画面）
//
// 【SPA(シングルページアプリケーション)の仕組み】
// ページ遷移時にHTTPリクエストは発生せず、JavaScriptが
// 表示するコンポーネントを切り替えるだけ。URLは変わるが
// サーバーには問い合わせない（history APIを利用）。
// ============================================================
import { createRouter, createWebHistory } from 'vue-router'
import UserView from '@/views/UserView.vue'
import OwnerView from '@/views/OwnerView.vue'

const router = createRouter({
  // createWebHistory: HTML5 History APIを使用（URLにハッシュ#が付かない）
  // サーバー側でどのパスにアクセスされても index.html を返す設定が必要
  // （Rustバックエンドの fallback ハンドラーがこれを担当している）
  history: createWebHistory(),

  routes: [
    {
      path: '/',
      name: 'user',
      component: UserView,
      meta: {
        title: 'チーム決め一発くん',
        isOwner: false,
      },
    },
    {
      path: '/owner',
      name: 'owner',
      component: OwnerView,
      meta: {
        title: 'チーム決め一発くん - 管理者',
        isOwner: true,
      },
    },
    {
      // 定義されていないパスはすべてユーザー画面にリダイレクト
      path: '/:pathMatch(.*)*',
      redirect: '/',
    },
  ],

  // ページ遷移時にスクロール位置をリセットする
  scrollBehavior(_to, _from, savedPosition) {
    if (savedPosition) {
      return savedPosition
    }
    return { top: 0 }
  },
})

// ── ナビゲーションガード ──
// ルート遷移前に呼ばれるフック。
// ここでページタイトルを動的に変更する。
router.beforeEach((to) => {
  if (to.meta.title) {
    document.title = to.meta.title as string
  }
})

export default router
