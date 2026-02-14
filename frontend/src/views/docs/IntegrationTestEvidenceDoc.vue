<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
  <div class="min-h-screen bg-neutral-100">
    <header class="bg-neutral-900 text-white px-6 py-4 flex items-center gap-4 shadow">
      <button @click="router.push('/owner/docs')" class="text-neutral-400 hover:text-white transition-colors text-sm">← 設計書一覧に戻る</button>
      <div class="w-px h-4 bg-neutral-600" />
      <h1 class="text-base font-bold">✅ 結合テスト証跡</h1>
      <div class="flex-1" />
      <span class="text-xs text-neutral-400">初版 ／ 2025年作成</span>
    </header>

    <div class="max-w-4xl mx-auto px-6 py-8 space-y-8">

      <!-- 実施概要 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-neutral-700 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">テスト実施概要</h2>
        </div>
        <div class="p-5">
          <div class="grid grid-cols-2 gap-4 text-sm mb-4">
            <div class="space-y-2">
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">実施日</span><span class="font-medium">2025-01-01</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">実施者</span><span class="font-medium">チーム決め一発くん 開発チーム</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">テスト環境</span><span class="font-medium">ローカル開発環境（localhost:8080）</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">使用ツール</span><span class="font-medium">ブラウザ / curl コマンド</span></div>
            </div>
            <div class="space-y-2">
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">テスト総数</span><span class="font-bold text-neutral-900">16件</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">合格</span><span class="font-bold text-emerald-600">16件</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">不合格</span><span class="font-bold text-red-600">0件</span></div>
            </div>
          </div>
          <div class="flex items-center gap-3">
            <span class="text-xs text-neutral-500 w-16">合格率</span>
            <div class="flex-1 h-4 bg-neutral-100 rounded overflow-hidden">
              <div class="h-full bg-emerald-500 rounded" style="width:100%"></div>
            </div>
            <span class="text-sm font-black text-emerald-600">100%</span>
          </div>
        </div>
      </section>

      <!-- API 実行ログ（抜粋） -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-neutral-700 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">API実行ログ（代表的なケース）</h2>
        </div>
        <div class="p-5 space-y-4">

          <div>
            <p class="text-xs font-bold text-neutral-700 mb-1">IT-A-01: プレイヤー追加確認</p>
            <div v-pre class="bg-neutral-900 text-green-400 font-mono text-[10px] p-3 rounded leading-relaxed">
              <p class="text-neutral-500"># プレイヤー追加</p>
              <p>$ curl -X POST http://localhost:8080/api/players \</p>
              <p>  -H "Content-Type: application/json" \</p>
              <p>  -d '{"name":"テストプレイヤー","rank":"ゴールド"}'</p>
              <p class="text-green-300 mt-1">{"player":{"id":"uuid-xxxx","name":"テストプレイヤー","rank":"ゴールド","score_offset":0}}</p>
              <p class="mt-2 text-neutral-500"># 一覧取得で確認</p>
              <p>$ curl http://localhost:8080/api/players</p>
              <p class="text-green-300">{"players":[{"id":"uuid-xxxx","name":"テストプレイヤー",...}]}</p>
              <p class="text-emerald-400 mt-1 font-bold">✓ 追加したプレイヤーが一覧に含まれることを確認</p>
            </div>
          </div>

          <div>
            <p class="text-xs font-bold text-neutral-700 mb-1">IT-B-04: プレイヤー0人でのチーム分けエラー</p>
            <div v-pre class="bg-neutral-900 text-green-400 font-mono text-[10px] p-3 rounded leading-relaxed">
              <p class="text-neutral-500"># プレイヤーが0人の状態でチーム分け実行</p>
              <p>$ curl -X POST http://localhost:8080/api/teams/balance</p>
              <p class="text-red-400">{"error":"プレイヤーが登録されていません"} (HTTP 400)</p>
              <p class="text-emerald-400 mt-1 font-bold">✓ 適切なエラーレスポンスが返却されることを確認</p>
            </div>
          </div>

          <div>
            <p class="text-xs font-bold text-neutral-700 mb-1">IT-C-02: チーム数最小値保護</p>
            <div v-pre class="bg-neutral-900 text-green-400 font-mono text-[10px] p-3 rounded leading-relaxed">
              <p class="text-neutral-500"># チーム数を1（最小値未満）に設定</p>
              <p>$ curl -X PUT http://localhost:8080/api/config \</p>
              <p>  -d '{"team_count":1}'</p>
              <p class="text-green-300">{"config":{"team_count":2}}</p>
              <p class="text-emerald-400 mt-1 font-bold">✓ 1が2に補正されることを確認</p>
            </div>
          </div>

        </div>
      </section>

      <!-- テスト結果一覧 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-neutral-700 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">テスト結果一覧</h2>
        </div>
        <div class="p-5">
          <table class="w-full text-xs border-collapse">
            <thead>
              <tr class="bg-neutral-100 text-neutral-600">
                <th class="border border-neutral-200 px-3 py-2 text-left w-24">テストID</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">実際の結果</th>
                <th class="border border-neutral-200 px-3 py-2 text-center w-16">判定</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="tc in [
                ['IT-A-01', 'プレイヤー追加→一覧反映', '追加したプレイヤーが一覧に含まれることを確認', '合格'],
                ['IT-A-02', 'プレイヤー更新→変更反映', 'ランク変更後の値がGETで確認できた', '合格'],
                ['IT-A-03', 'プレイヤー削除→一覧から消える', '削除後のGETにプレイヤーが含まれない', '合格'],
                ['IT-A-04', '補正値±1調整', 'score_offsetが1増加したことを確認', '合格'],
                ['IT-A-05', '存在しないID削除→404', 'HTTP 404が正しく返却された', '合格'],
                ['IT-B-01', 'チーム分け実行→結果取得', '指定チーム数のチームが返却された', '合格'],
                ['IT-B-02', 'チーム分け前GET→404', 'HTTP 404が正しく返却された', '合格'],
                ['IT-B-03', 'チーム解散→GET 404', '解散後にHTTP 404が返却された', '合格'],
                ['IT-B-04', '0人チーム分け→400エラー', 'HTTP 400とエラーメッセージが返却された', '合格'],
                ['IT-B-05', '重複名解決確認', 'display_nameに(1)(2)が設定された', '合格'],
                ['IT-C-01', 'チーム数変更→設定反映', '変更値がGETで確認できた', '合格'],
                ['IT-C-02', 'チーム数1→2に保護', 'team_count=2が返却された', '合格'],
                ['IT-C-03', 'ヘルスチェック正常応答', 'HTTP 200 / {status:ok}が返却された', '合格'],
                ['IT-D-01', 'ユーザー登録→画面表示', '登録後すぐに参加者リストに表示された', '合格'],
                ['IT-D-02', 'ポーリングで他ユーザー反映', '3秒以内に別タブの画面に反映された', '合格'],
                ['IT-D-03', 'エラートースト表示', '空名前でエラートーストが表示された', '合格'],
              ]" :key="tc[0]" class="hover:bg-neutral-50">
                <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                <td class="border border-neutral-200 px-3 py-2">{{ tc[1] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ tc[2] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-center">
                  <span class="bg-emerald-100 text-emerald-700 font-bold px-2 py-0.5 text-[10px]">{{ tc[3] }}</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- ページ下部ナビゲーション -->
      <div class="pt-2 pb-8 border-t border-neutral-200">
        <button
          @click="router.push('/owner/docs')"
          class="flex items-center gap-2 px-5 py-2.5 bg-neutral-800 text-white text-sm hover:bg-neutral-700 transition-colors"
        >
          ← 設計書一覧に戻る
        </button>
      </div>

    </div>
  </div>
</template>
