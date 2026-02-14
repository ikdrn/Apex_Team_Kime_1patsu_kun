<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
  <div class="min-h-screen bg-neutral-100">
    <header class="bg-violet-700 text-white px-6 py-4 flex items-center gap-4 shadow">
      <button @click="router.push('/owner/docs')" class="text-violet-200 hover:text-white transition-colors text-sm">← 設計書一覧に戻る</button>
      <div class="w-px h-4 bg-violet-500" />
      <h1 class="text-base font-bold">✅ 総合テスト証跡</h1>
      <div class="flex-1" />
      <span class="text-xs text-violet-200">バージョン 1.0 ／ 2025年作成</span>
    </header>

    <div class="max-w-4xl mx-auto px-6 py-8 space-y-8">

      <!-- 実施概要 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-violet-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">テスト実施概要</h2>
        </div>
        <div class="p-5">
          <div class="grid grid-cols-2 gap-4 text-sm mb-4">
            <div class="space-y-2">
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">実施日</span><span class="font-medium">2025-01-01</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">実施者</span><span class="font-medium">チーム決め一発くん 開発チーム（オーナー役 + 参加者役 各1名）</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">テスト環境</span><span class="font-medium">ローカル開発環境 / Chrome ブラウザ</span></div>
            </div>
            <div class="space-y-2">
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">シナリオ数</span><span class="font-bold text-neutral-900">4シナリオ</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">テストケース数</span><span class="font-bold text-neutral-900">8件</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-28 shrink-0">合格</span><span class="font-bold text-emerald-600">8件</span></div>
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

      <!-- シナリオ実施記録 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-violet-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">シナリオ実施記録</h2>
        </div>
        <div class="p-5 space-y-5">

          <!-- シナリオ1 -->
          <div class="border border-neutral-200 p-4">
            <div class="flex items-center justify-between mb-3">
              <div class="flex items-center gap-2">
                <span class="bg-violet-100 text-violet-700 text-xs font-bold px-2 py-0.5">シナリオ 1</span>
                <span class="text-sm font-bold">6人3チームの基本フロー</span>
              </div>
              <span class="bg-emerald-100 text-emerald-700 text-xs font-bold px-2 py-0.5">合格</span>
            </div>
            <div class="space-y-2 text-xs">
              <div v-for="step in [
                {no:1, result:'管理者URL（/owner）にアクセスし、管理者画面が正常に表示された'},
                {no:2, result:'チーム数セレクトボックスで3を選択し、ヘッダーの「チーム」表示が3に更新された'},
                {no:3, result:'自分を追加フォームで「田中/ゴールド」を登録。参加者リストに即時反映された'},
                {no:4, result:'別タブでユーザーURL（/）を開き、「鈴木/マスター」で登録。管理者画面にも3秒以内に反映された'},
                {no:5, result:'同様に4名が追加登録され、参加者数が6名になった'},
                {no:6, result:'「チーム分けを実行」を押下。3チームのカードが表示された（スコア差7で「均衡」判定）'},
                {no:7, result:'ユーザー画面で3秒以内にチーム分け結果が表示。登録した「鈴木」に★が付いた'},
                {no:8, result:'「チーム解散」を押下。「チーム分けがまだ実行されていません」状態に戻った'},
              ]" :key="step.no" class="flex gap-2">
                <span class="text-violet-500 font-bold shrink-0">{{ step.no }}.</span>
                <span class="text-neutral-700">{{ step.result }}</span>
              </div>
            </div>
          </div>

          <!-- シナリオ2 -->
          <div class="border border-neutral-200 p-4">
            <div class="flex items-center justify-between mb-3">
              <div class="flex items-center gap-2">
                <span class="bg-violet-100 text-violet-700 text-xs font-bold px-2 py-0.5">シナリオ 2</span>
                <span class="text-sm font-bold">補正値調整フロー</span>
              </div>
              <span class="bg-emerald-100 text-emerald-700 text-xs font-bold px-2 py-0.5">合格</span>
            </div>
            <div class="space-y-2 text-xs">
              <div v-for="step in [
                {no:1, result:'「上田（プレデター/スコア70）」を管理者が追加。参加者リストに表示された'},
                {no:2, result:'「−」ボタンを2回押下。補正値欄に「68 (−2)」と表示された（API呼び出し2回）'},
                {no:3, result:'チーム分けを実行。上田のスコアが70ではなく68として計算されたチームが生成された'},
              ]" :key="step.no" class="flex gap-2">
                <span class="text-violet-500 font-bold shrink-0">{{ step.no }}.</span>
                <span class="text-neutral-700">{{ step.result }}</span>
              </div>
            </div>
          </div>

          <!-- シナリオ3 -->
          <div class="border border-neutral-200 p-4">
            <div class="flex items-center justify-between mb-3">
              <div class="flex items-center gap-2">
                <span class="bg-violet-100 text-violet-700 text-xs font-bold px-2 py-0.5">シナリオ 3</span>
                <span class="text-sm font-bold">同名プレイヤーの重複名解決</span>
              </div>
              <span class="bg-emerald-100 text-emerald-700 text-xs font-bold px-2 py-0.5">合格</span>
            </div>
            <div class="space-y-2 text-xs">
              <div v-for="step in [
                {no:1, result:'「さとう」を2名が別々に登録。参加者リストには「さとう」が2件表示された'},
                {no:2, result:'チーム分けを実行。チーム結果には「さとう(1)」「さとう(2)」と正しく区別されて表示された'},
              ]" :key="step.no" class="flex gap-2">
                <span class="text-violet-500 font-bold shrink-0">{{ step.no }}.</span>
                <span class="text-neutral-700">{{ step.result }}</span>
              </div>
            </div>
          </div>

          <!-- シナリオ4 -->
          <div class="border border-amber-200 p-4 bg-amber-50">
            <div class="flex items-center justify-between mb-3">
              <div class="flex items-center gap-2">
                <span class="bg-amber-500 text-white text-xs font-bold px-2 py-0.5">シナリオ 4</span>
                <span class="text-sm font-bold">異常系：0人でチーム分け実行</span>
              </div>
              <span class="bg-emerald-100 text-emerald-700 text-xs font-bold px-2 py-0.5">合格</span>
            </div>
            <div class="space-y-2 text-xs">
              <div v-for="step in [
                {no:1, result:'参加者0人の状態では「チーム分けを実行」ボタンが非活性（グレーアウト）になっていた'},
                {no:2, result:'curlでAPIを直接叩いたところ、HTTP 400とエラーメッセージが返却され、画面にもトーストが表示された'},
              ]" :key="step.no" class="flex gap-2">
                <span class="text-amber-600 font-bold shrink-0">{{ step.no }}.</span>
                <span class="text-neutral-700">{{ step.result }}</span>
              </div>
            </div>
          </div>

        </div>
      </section>

      <!-- テスト結果一覧 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-violet-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">テスト結果一覧</h2>
        </div>
        <div class="p-5">
          <table class="w-full text-xs border-collapse">
            <thead>
              <tr class="bg-neutral-100 text-neutral-600">
                <th class="border border-neutral-200 px-3 py-2 text-left w-20">テストID</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">実際の結果</th>
                <th class="border border-neutral-200 px-3 py-2 text-center w-16">判定</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="tc in [
                ['ST-01', '6人3チームの基本フロー全体', 'シナリオ1の全ステップが期待通りに動作した', '合格'],
                ['ST-02', '補正値調整後のチーム分け', '補正値68がチーム分けスコアに反映された', '合格'],
                ['ST-03', '同名プレイヤーの重複名解決', 'さとう(1)・さとう(2)が正しく表示された', '合格'],
                ['ST-04', '0人チーム分けの防御', 'ボタン非活性・APIエラー両方とも正常に動作', '合格'],
                ['ST-05', '設計書へのアクセス', '管理者画面ヘッダーの「設計書」ボタンから設計書一覧に遷移できた', '合格'],
                ['ST-06', '設計書のアクセス制限確認', 'ユーザー画面（/）には設計書へのリンクが存在しない', '合格'],
                ['ST-07', '参加者数不足の警告表示', '参加者2人・チーム数3のとき警告メッセージが表示された', '合格'],
                ['ST-08', '自分の★表示', 'ユーザー画面で自分のプレイヤー名横に★が表示された', '合格'],
              ]" :key="tc[0]" class="hover:bg-neutral-50">
                <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                <td class="border border-neutral-200 px-3 py-2 font-medium">{{ tc[1] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ tc[2] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-center">
                  <span class="bg-emerald-100 text-emerald-700 font-bold px-2 py-0.5 text-[10px]">{{ tc[3] }}</span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- 総評 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-violet-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">総評・リリース判定</h2>
        </div>
        <div class="p-5 space-y-3">
          <div class="p-4 bg-emerald-50 border border-emerald-300">
            <div class="flex items-center gap-3 mb-2">
              <span class="text-2xl">✅</span>
              <span class="text-lg font-black text-emerald-700">リリース承認</span>
            </div>
            <p class="text-sm text-neutral-700 leading-relaxed">
              全8件の総合テストが合格しました。4つのシナリオすべてにおいて期待通りの動作が確認できました。
              コードレビューで発見された不具合（テストアサーション値の誤り・コメント不一致・ポーリング未実装・grid-cols重複バグ・バリデーション不足・prop命名の不明確さ）は
              すべて修正済みです。本システムのリリースを承認します。
            </p>
          </div>
          <div class="p-3 bg-amber-50 border border-amber-200 text-xs text-amber-800">
            <p class="font-bold mb-1">⚠ 既知の制限事項（次バージョンで対応予定）</p>
            <ul class="space-y-1 text-amber-700">
              <li>• データ永続化なし（サーバー再起動でリセット）→ データベース導入が推奨</li>
              <li>• 認証機能なし（URLを知れば誰でも管理者画面にアクセス可能）→ パスワード認証の追加が推奨</li>
              <li>• ポーリング方式（3秒遅延）→ WebSocketによるリアルタイム通信への移行が推奨</li>
            </ul>
          </div>
        </div>
      </section>

    </div>
  </div>
</template>
