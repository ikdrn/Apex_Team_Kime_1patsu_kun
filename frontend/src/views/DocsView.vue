<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()

const docs = [
  {
    category: '要件・設計',
    color: 'blue',
    items: [
      { name: '要件定義書', desc: 'このシステムが「何のために」「誰が使うか」「何ができるか」をまとめた文書', route: 'docs-requirements', icon: '📋' },
      { name: '基本設計書', desc: 'システム全体の仕組みと大まかな構造を図で示した文書', route: 'docs-basic-design', icon: '🏗️' },
      { name: '詳細設計書', desc: '各機能の細かい動作仕様・データ構造・アルゴリズムを記載した文書', route: 'docs-detailed-design', icon: '⚙️' },
    ],
  },
  {
    category: '単体テスト',
    color: 'emerald',
    items: [
      { name: '単体テスト仕様書', desc: '個々の機能（部品）を単独でテストするための計画と手順', route: 'docs-unit-test-spec', icon: '🧪' },
      { name: '単体テスト証跡', desc: '単体テストを実施した記録と合否結果', route: 'docs-unit-test-evidence', icon: '✅' },
    ],
  },
  {
    category: '結合テスト',
    color: 'amber',
    items: [
      { name: '結合テスト仕様書', desc: '複数の機能を組み合わせてテストするための計画と手順', route: 'docs-integration-test-spec', icon: '🔗' },
      { name: '結合テスト証跡', desc: '結合テストを実施した記録と合否結果', route: 'docs-integration-test-evidence', icon: '✅' },
    ],
  },
  {
    category: '総合テスト',
    color: 'violet',
    items: [
      { name: '総合テスト仕様書', desc: '実際の利用シナリオに沿ってシステム全体をテストするための計画と手順', route: 'docs-system-test-spec', icon: '🎯' },
      { name: '総合テスト証跡', desc: '総合テストを実施した記録と合否結果', route: 'docs-system-test-evidence', icon: '✅' },
    ],
  },
]

const colorMap: Record<string, string> = {
  blue:    'border-blue-200 bg-blue-50 hover:bg-blue-100',
  emerald: 'border-emerald-200 bg-emerald-50 hover:bg-emerald-100',
  amber:   'border-amber-200 bg-amber-50 hover:bg-amber-100',
  violet:  'border-violet-200 bg-violet-50 hover:bg-violet-100',
}
const headerMap: Record<string, string> = {
  blue:    'bg-blue-600',
  emerald: 'bg-emerald-600',
  amber:   'bg-amber-500',
  violet:  'bg-violet-600',
}
</script>

<template>
  <div class="min-h-screen bg-neutral-100">
    <!-- ヘッダー -->
    <header class="bg-neutral-900 text-white px-6 py-4 flex items-center gap-4 shadow">
      <button @click="router.push('/owner')" class="text-neutral-400 hover:text-white transition-colors text-sm">← 管理者画面に戻る</button>
      <div class="w-px h-4 bg-neutral-600" />
      <h1 class="text-base font-bold">設計書一覧</h1>
      <span class="text-[10px] font-semibold tracking-widest uppercase text-neutral-400 border border-neutral-700 px-1.5 py-0.5">OWNER ONLY</span>
    </header>

    <!-- コンテンツ -->
    <div class="max-w-4xl mx-auto px-6 py-8">

      <!-- 注意書き -->
      <div class="mb-8 p-4 bg-amber-50 border border-amber-200 text-sm text-amber-800 flex gap-2 items-start">
        <span class="text-lg leading-none mt-0.5">⚠️</span>
        <div>
          <p class="font-semibold">この設計書は管理者（オーナー）専用です。</p>
          <p class="text-xs mt-1 text-amber-700">一般ユーザーには共有しないでください。URL直打ちでのアクセスは避け、必ず管理者画面の「設計書」ボタンからアクセスしてください。</p>
        </div>
      </div>

      <!-- プロジェクト概要 -->
      <div class="mb-8 bg-white border border-neutral-200 shadow-sm p-6">
        <h2 class="text-lg font-bold text-neutral-900 mb-2">チーム決め一発くん</h2>
        <p class="text-sm text-neutral-600 leading-relaxed">
          Apex Legends のカスタムマッチで使用する「公平なチーム分け」を自動で行うWebアプリケーション。<br>
          参加者のランクと個人差補正値をもとに、戦闘力のバランスが均等になるようにチームを組み分けます。
        </p>
        <div class="mt-4 flex gap-6">
          <div class="text-center">
            <div class="text-2xl font-black text-neutral-900">v4</div>
            <div class="text-xs text-neutral-400 mt-0.5">バージョン</div>
          </div>
          <div class="text-center">
            <div class="text-2xl font-black text-neutral-900">2</div>
            <div class="text-xs text-neutral-400 mt-0.5">ユーザー種別</div>
          </div>
          <div class="text-center">
            <div class="text-2xl font-black text-neutral-900">9</div>
            <div class="text-xs text-neutral-400 mt-0.5">API エンドポイント</div>
          </div>
          <div class="text-center">
            <div class="text-2xl font-black text-neutral-900">30</div>
            <div class="text-xs text-neutral-400 mt-0.5">チーム分け試行回数</div>
          </div>
        </div>
      </div>

      <!-- 文書一覧 -->
      <div class="space-y-6">
        <div v-for="cat in docs" :key="cat.category">
          <div class="flex items-center gap-3 mb-3">
            <div :class="[headerMap[cat.color], 'px-3 py-1 text-xs font-bold text-white tracking-wider']">
              {{ cat.category }}
            </div>
            <div class="flex-1 h-px bg-neutral-200" />
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            <button
              v-for="doc in cat.items"
              :key="doc.name"
              @click="router.push({ name: doc.route })"
              :class="['border p-4 text-left transition-colors', colorMap[cat.color]]"
            >
              <div class="flex items-start gap-3">
                <span class="text-2xl leading-none">{{ doc.icon }}</span>
                <div>
                  <p class="font-bold text-neutral-900 text-sm">{{ doc.name }}</p>
                  <p class="text-xs text-neutral-600 mt-1 leading-relaxed">{{ doc.desc }}</p>
                </div>
              </div>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
