<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
  <div class="min-h-screen bg-neutral-100">
    <header class="bg-amber-600 text-white px-6 py-4 flex items-center gap-4 shadow">
      <button @click="router.push('/owner/docs')" class="text-amber-200 hover:text-white transition-colors text-sm">← 設計書一覧に戻る</button>
      <div class="w-px h-4 bg-amber-400" />
      <h1 class="text-base font-bold">🔗 結合テスト仕様書</h1>
      <div class="flex-1" />
      <span class="text-xs text-amber-200">バージョン 1.0 ／ 2025年作成</span>
    </header>

    <div class="max-w-4xl mx-auto px-6 py-8 space-y-8">

      <!-- テスト方針 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-amber-500 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">1. テスト方針</h2>
        </div>
        <div class="p-5 space-y-3">
          <div class="p-4 bg-amber-50 border-l-4 border-amber-400">
            <p class="text-sm font-semibold text-amber-800 mb-1">🎯 結合テストとは？</p>
            <p class="text-sm text-neutral-700">
              「部品を組み合わせたときに正しく連携して動くか」を確認するテストです。
              フロントエンド（画面）とバックエンド（サーバー）が、実際にAPIを通じて正しくデータをやりとりできるかを検証します。
            </p>
          </div>
          <div class="grid grid-cols-3 gap-3 text-xs">
            <div class="border border-neutral-200 p-3 bg-neutral-50">
              <p class="font-bold text-neutral-700 mb-1">テスト対象</p>
              <p class="text-neutral-600">全APIエンドポイント（9種）とフロントエンドの連携</p>
            </div>
            <div class="border border-neutral-200 p-3 bg-neutral-50">
              <p class="font-bold text-neutral-700 mb-1">テスト環境</p>
              <p class="text-neutral-600">ローカル開発環境（バックエンド:8080番ポート）</p>
            </div>
            <div class="border border-neutral-200 p-3 bg-neutral-50">
              <p class="font-bold text-neutral-700 mb-1">テスト手法</p>
              <p class="text-neutral-600">実際にブラウザとAPIツール（curl等）で手動確認</p>
            </div>
          </div>
        </div>
      </section>

      <!-- API連携テスト -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-amber-500 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">2. APIエンドポイント連携テスト</h2>
        </div>
        <div class="p-5 space-y-5">

          <!-- グループ1: プレイヤー管理 -->
          <div>
            <div class="flex items-center gap-2 mb-3">
              <span class="bg-blue-100 text-blue-700 text-xs font-bold px-2 py-0.5">グループA</span>
              <span class="text-sm font-bold text-neutral-800">プレイヤー管理API</span>
            </div>
            <table class="w-full text-xs border-collapse">
              <thead>
                <tr class="bg-neutral-100 text-neutral-600">
                  <th class="border border-neutral-200 px-3 py-2 text-left w-20">テストID</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">操作手順</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">期待される結果</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="tc in [
                  ['IT-A-01', 'プレイヤー追加後、一覧に反映されること', 'POST /api/players → GET /api/players', '追加したプレイヤーがリストに含まれる'],
                  ['IT-A-02', 'プレイヤー更新後、変更が反映されること', 'PUT /api/players/:id でランク変更 → GET /api/players', '更新されたランクが取得できる'],
                  ['IT-A-03', 'プレイヤー削除後、一覧から消えること', 'DELETE /api/players/:id → GET /api/players', '削除したプレイヤーがリストに含まれない'],
                  ['IT-A-04', '補正値調整後、スコアが変化すること', 'PATCH /api/players/:id/offset (delta:+1) → GET /api/players', 'score_offset が 1 増加している'],
                  ['IT-A-05', '存在しないIDを削除するとエラーになること', 'DELETE /api/players/存在しないID', 'HTTP 404 / エラーメッセージ返却'],
                ]" :key="tc[0]" class="hover:bg-neutral-50">
                  <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                  <td class="border border-neutral-200 px-3 py-2">{{ tc[1] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-neutral-600 font-mono text-[10px]">{{ tc[2] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-amber-700 font-medium">{{ tc[3] }}</td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- グループ2: チーム管理 -->
          <div>
            <div class="flex items-center gap-2 mb-3">
              <span class="bg-orange-100 text-orange-700 text-xs font-bold px-2 py-0.5">グループB</span>
              <span class="text-sm font-bold text-neutral-800">チーム分けAPI</span>
            </div>
            <table class="w-full text-xs border-collapse">
              <thead>
                <tr class="bg-neutral-100 text-neutral-600">
                  <th class="border border-neutral-200 px-3 py-2 text-left w-20">テストID</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">操作手順</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">期待される結果</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="tc in [
                  ['IT-B-01', 'チーム分け実行後、チーム結果が取得できること', 'POST /api/teams/balance → GET /api/teams', '指定チーム数のチームが返ってくる'],
                  ['IT-B-02', 'チーム分け前はGET /api/teamsが404を返すこと', 'サーバー起動直後にGET /api/teams', 'HTTP 404 返却（エラーではなく未実行の意味）'],
                  ['IT-B-03', 'チーム解散後、GETで404になること', 'POST balance → DELETE /api/teams → GET /api/teams', 'HTTP 404 返却'],
                  ['IT-B-04', 'プレイヤー0人でチーム分けするとエラーになること', 'POST /api/teams/balance（プレイヤーなし）', 'HTTP 400 / エラーメッセージ返却'],
                  ['IT-B-05', 'チーム分け後に重複名が解決されること', '同名プレイヤー2人 → POST balance', 'playersに display_name が設定されている'],
                ]" :key="tc[0]" class="hover:bg-neutral-50">
                  <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                  <td class="border border-neutral-200 px-3 py-2">{{ tc[1] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-neutral-600 font-mono text-[10px]">{{ tc[2] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-amber-700 font-medium">{{ tc[3] }}</td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- グループ3: 設定API -->
          <div>
            <div class="flex items-center gap-2 mb-3">
              <span class="bg-emerald-100 text-emerald-700 text-xs font-bold px-2 py-0.5">グループC</span>
              <span class="text-sm font-bold text-neutral-800">設定API / ヘルスチェック</span>
            </div>
            <table class="w-full text-xs border-collapse">
              <thead>
                <tr class="bg-neutral-100 text-neutral-600">
                  <th class="border border-neutral-200 px-3 py-2 text-left w-20">テストID</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">操作手順</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">期待される結果</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="tc in [
                  ['IT-C-01', 'チーム数変更後、設定が反映されること', 'PUT /api/config → GET /api/config', '変更したteam_countが取得できる'],
                  ['IT-C-02', 'チーム数を1にしても2になること（最小値保護）', 'PUT /api/config (team_count:1)', 'team_count = 2 が返却される'],
                  ['IT-C-03', 'ヘルスチェックが正常応答すること', 'GET /health', 'HTTP 200 / {&quot;status&quot;:&quot;ok&quot;} 返却'],
                ]" :key="tc[0]" class="hover:bg-neutral-50">
                  <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                  <td class="border border-neutral-200 px-3 py-2">{{ tc[1] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-neutral-600 font-mono text-[10px]">{{ tc[2] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-amber-700 font-medium">{{ tc[3] }}</td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- グループ4: フロントエンド連携 -->
          <div>
            <div class="flex items-center gap-2 mb-3">
              <span class="bg-violet-100 text-violet-700 text-xs font-bold px-2 py-0.5">グループD</span>
              <span class="text-sm font-bold text-neutral-800">フロントエンド ↔ バックエンド連携</span>
            </div>
            <table class="w-full text-xs border-collapse">
              <thead>
                <tr class="bg-neutral-100 text-neutral-600">
                  <th class="border border-neutral-200 px-3 py-2 text-left w-20">テストID</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">操作手順</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">期待される結果</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="tc in [
                  ['IT-D-01', '一般ユーザーが登録すると画面に表示されること', 'ユーザー画面でフォーム入力→登録ボタン', '参加者リストにすぐ表示される'],
                  ['IT-D-02', 'ポーリングで他ユーザーの登録が反映されること', 'Aユーザーが登録 → Bユーザーの画面で3秒待つ', 'Aユーザーの名前がBの画面にも表示される'],
                  ['IT-D-03', 'エラー時にトーストが表示されること', '空名前で登録ボタン押下', '画面下部にエラートーストが5秒間表示される'],
                ]" :key="tc[0]" class="hover:bg-neutral-50">
                  <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                  <td class="border border-neutral-200 px-3 py-2">{{ tc[1] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ tc[2] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-amber-700 font-medium">{{ tc[3] }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </section>

    </div>
  </div>
</template>
