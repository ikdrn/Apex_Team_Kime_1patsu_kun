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
import DocsView from '@/views/DocsView.vue'
import RequirementsDoc from '@/views/docs/RequirementsDoc.vue'
import BasicDesignDoc from '@/views/docs/BasicDesignDoc.vue'
import DetailedDesignDoc from '@/views/docs/DetailedDesignDoc.vue'
import UnitTestSpecDoc from '@/views/docs/UnitTestSpecDoc.vue'
import UnitTestEvidenceDoc from '@/views/docs/UnitTestEvidenceDoc.vue'
import IntegrationTestSpecDoc from '@/views/docs/IntegrationTestSpecDoc.vue'
import IntegrationTestEvidenceDoc from '@/views/docs/IntegrationTestEvidenceDoc.vue'
import SystemTestSpecDoc from '@/views/docs/SystemTestSpecDoc.vue'
import SystemTestEvidenceDoc from '@/views/docs/SystemTestEvidenceDoc.vue'

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
    // ── 設計書（管理者専用） ──
    {
      path: '/owner/docs',
      name: 'docs',
      component: DocsView,
      meta: { title: '設計書一覧 - チーム決め一発くん', isOwner: true },
    },
    {
      path: '/owner/docs/requirements',
      name: 'docs-requirements',
      component: RequirementsDoc,
      meta: { title: '要件定義書', isOwner: true },
    },
    {
      path: '/owner/docs/basic-design',
      name: 'docs-basic-design',
      component: BasicDesignDoc,
      meta: { title: '基本設計書', isOwner: true },
    },
    {
      path: '/owner/docs/detailed-design',
      name: 'docs-detailed-design',
      component: DetailedDesignDoc,
      meta: { title: '詳細設計書', isOwner: true },
    },
    {
      path: '/owner/docs/unit-test-spec',
      name: 'docs-unit-test-spec',
      component: UnitTestSpecDoc,
      meta: { title: '単体テスト仕様書', isOwner: true },
    },
    {
      path: '/owner/docs/unit-test-evidence',
      name: 'docs-unit-test-evidence',
      component: UnitTestEvidenceDoc,
      meta: { title: '単体テスト証跡', isOwner: true },
    },
    {
      path: '/owner/docs/integration-test-spec',
      name: 'docs-integration-test-spec',
      component: IntegrationTestSpecDoc,
      meta: { title: '結合テスト仕様書', isOwner: true },
    },
    {
      path: '/owner/docs/integration-test-evidence',
      name: 'docs-integration-test-evidence',
      component: IntegrationTestEvidenceDoc,
      meta: { title: '結合テスト証跡', isOwner: true },
    },
    {
      path: '/owner/docs/system-test-spec',
      name: 'docs-system-test-spec',
      component: SystemTestSpecDoc,
      meta: { title: '総合テスト仕様書', isOwner: true },
    },
    {
      path: '/owner/docs/system-test-evidence',
      name: 'docs-system-test-evidence',
      component: SystemTestEvidenceDoc,
      meta: { title: '総合テスト証跡', isOwner: true },
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
