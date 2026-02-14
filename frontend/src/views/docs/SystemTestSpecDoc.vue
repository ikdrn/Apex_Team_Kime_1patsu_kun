<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
  <div class="min-h-screen bg-neutral-100">
    <header class="bg-violet-700 text-white px-6 py-4 flex items-center gap-4 shadow">
      <button @click="router.push('/owner/docs')" class="text-violet-200 hover:text-white transition-colors text-sm">← 設計書一覧に戻る</button>
      <div class="w-px h-4 bg-violet-500" />
      <h1 class="text-base font-bold">🎯 総合テスト仕様書</h1>
      <div class="flex-1" />
      <span class="text-xs text-violet-200">バージョン 1.0 ／ 2025年作成</span>
    </header>

    <div class="max-w-4xl mx-auto px-6 py-8 space-y-8">

      <!-- テスト方針 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-violet-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">1. テスト方針</h2>
        </div>
        <div class="p-5 space-y-3">
          <div class="p-4 bg-violet-50 border-l-4 border-violet-400">
            <p class="text-sm font-semibold text-violet-800 mb-1">🎯 総合テストとは？</p>
            <p class="text-sm text-neutral-700">
              実際の利用シナリオに沿ってシステム全体を動かす最終確認テストです。
              「実際のカスタムマッチ当日に使うように操作して、問題なく使えるか」を確認します。
              エンジニア以外の視点（ユーザー目線）でテストを行います。
            </p>
          </div>
          <div class="grid grid-cols-3 gap-3 text-xs">
            <div class="border border-neutral-200 p-3 bg-neutral-50">
              <p class="font-bold text-neutral-700 mb-1">テスト範囲</p>
              <p class="text-neutral-600">システム全体のエンドツーエンド（E2E）シナリオ</p>
            </div>
            <div class="border border-neutral-200 p-3 bg-neutral-50">
              <p class="font-bold text-neutral-700 mb-1">テスト実施者</p>
              <p class="text-neutral-600">オーナー（管理者）および参加者役のテスター</p>
            </div>
            <div class="border border-neutral-200 p-3 bg-neutral-50">
              <p class="font-bold text-neutral-700 mb-1">合格基準</p>
              <p class="text-neutral-600">全シナリオが期待通りに動作し、エラーが発生しないこと</p>
            </div>
          </div>
        </div>
      </section>

      <!-- シナリオ1: 基本的なカスタムマッチ当日の流れ -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-violet-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">2. テストシナリオ</h2>
        </div>
        <div class="p-5 space-y-6">

          <!-- シナリオ1 -->
          <div class="border border-violet-200 p-4">
            <div class="flex items-center gap-2 mb-4">
              <span class="bg-violet-600 text-white text-xs font-bold px-2 py-1">シナリオ 1</span>
              <span class="text-sm font-bold text-neutral-800">基本フロー：6人で3チームに分ける</span>
            </div>
            <div class="space-y-2">
              <div v-for="(step, idx) in [
                {actor: '管理者', action: '管理者URL（/owner）にアクセスする', check: '管理者画面が表示されること'},
                {actor: '管理者', action: '「チーム数」を3に変更する', check: '画面右上の「チーム」表示が3になること'},
                {actor: '管理者', action: '「自分を追加」ボタンを押し、名前「田中」ランク「ゴールド」で登録', check: '参加者リストに「田中」が追加されること'},
                {actor: '参加者A', action: 'ユーザーURL（/）にアクセスし、名前「鈴木」ランク「マスター」で登録', check: '参加者リストに「鈴木」が追加されること'},
                {actor: '参加者B〜E', action: '同様に4名が登録する', check: '参加者数が6名になること'},
                {actor: '管理者', action: '「チーム分けを実行」ボタンを押す', check: '3チームに分かれた結果が表示されること'},
                {actor: '参加者', action: 'ユーザー画面でチーム結果を確認する', check: '3秒以内にチーム分け結果が表示され、★が自分に表示されること'},
                {actor: '管理者', action: '「チーム解散」ボタンを押す', check: 'チーム結果が消え、未実行状態に戻ること'},
              ]" :key="idx" class="flex gap-3 text-xs">
                <div class="flex-none flex flex-col items-center">
                  <div class="w-6 h-6 rounded-full bg-violet-600 text-white flex items-center justify-center text-[10px] font-bold">{{ idx + 1 }}</div>
                  <div v-if="idx < 7" class="w-px flex-1 bg-violet-200 my-1" />
                </div>
                <div class="pb-3 flex-1">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="bg-neutral-200 text-neutral-700 font-bold px-1.5 py-0.5 text-[10px]">{{ step.actor }}</span>
                    <span class="text-neutral-800 font-medium">{{ step.action }}</span>
                  </div>
                  <div class="flex gap-1 items-start text-[10px] text-neutral-500">
                    <span class="text-violet-500 font-bold shrink-0">確認:</span>
                    <span>{{ step.check }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- シナリオ2 -->
          <div class="border border-violet-200 p-4">
            <div class="flex items-center gap-2 mb-4">
              <span class="bg-violet-600 text-white text-xs font-bold px-2 py-1">シナリオ 2</span>
              <span class="text-sm font-bold text-neutral-800">補正値調整フロー：強い人と弱い人がいる場合</span>
            </div>
            <div class="space-y-2">
              <div v-for="(step, idx) in [
                {actor: '管理者', action: '参加者「上田（プレデター）」を追加する', check: '参加者リストにプレデター「上田」が表示されること'},
                {actor: '管理者', action: '「上田」の戦闘力補正「−」ボタンを2回押す（補正値を−2にする）', check: '補正値が「(−2)」と表示され、実効スコアが68になること'},
                {actor: '管理者', action: '「チーム分けを実行」ボタンを押す', check: '補正値が反映されたスコアでチームが組まれること'},
              ]" :key="idx" class="flex gap-3 text-xs">
                <div class="flex-none flex flex-col items-center">
                  <div class="w-6 h-6 rounded-full bg-violet-600 text-white flex items-center justify-center text-[10px] font-bold">{{ idx + 1 }}</div>
                  <div v-if="idx < 2" class="w-px flex-1 bg-violet-200 my-1" />
                </div>
                <div class="pb-3 flex-1">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="bg-neutral-200 text-neutral-700 font-bold px-1.5 py-0.5 text-[10px]">{{ step.actor }}</span>
                    <span class="text-neutral-800 font-medium">{{ step.action }}</span>
                  </div>
                  <div class="flex gap-1 items-start text-[10px] text-neutral-500">
                    <span class="text-violet-500 font-bold shrink-0">確認:</span>
                    <span>{{ step.check }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- シナリオ3 -->
          <div class="border border-violet-200 p-4">
            <div class="flex items-center gap-2 mb-4">
              <span class="bg-violet-600 text-white text-xs font-bold px-2 py-1">シナリオ 3</span>
              <span class="text-sm font-bold text-neutral-800">同名プレイヤーフロー：同じ名前が複数いる場合</span>
            </div>
            <div class="space-y-2">
              <div v-for="(step, idx) in [
                {actor: '参加者', action: '「さとう」という名前で2人が別々に登録する', check: '参加者リストに「さとう」が2人表示されること'},
                {actor: '管理者', action: '「チーム分けを実行」ボタンを押す', check: 'チーム結果に「さとう(1)」「さとう(2)」と区別して表示されること'},
              ]" :key="idx" class="flex gap-3 text-xs">
                <div class="flex-none flex flex-col items-center">
                  <div class="w-6 h-6 rounded-full bg-violet-600 text-white flex items-center justify-center text-[10px] font-bold">{{ idx + 1 }}</div>
                  <div v-if="idx < 1" class="w-px flex-1 bg-violet-200 my-1" />
                </div>
                <div class="pb-3 flex-1">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="bg-neutral-200 text-neutral-700 font-bold px-1.5 py-0.5 text-[10px]">{{ step.actor }}</span>
                    <span class="text-neutral-800 font-medium">{{ step.action }}</span>
                  </div>
                  <div class="flex gap-1 items-start text-[10px] text-neutral-500">
                    <span class="text-violet-500 font-bold shrink-0">確認:</span>
                    <span>{{ step.check }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- シナリオ4 -->
          <div class="border border-amber-200 p-4 bg-amber-50">
            <div class="flex items-center gap-2 mb-4">
              <span class="bg-amber-500 text-white text-xs font-bold px-2 py-1">シナリオ 4</span>
              <span class="text-sm font-bold text-neutral-800">異常系：参加者が0人でチーム分けを実行</span>
            </div>
            <div class="space-y-2">
              <div v-for="(step, idx) in [
                {actor: '管理者', action: '誰も登録されていない状態で「チーム分けを実行」ボタンを押す', check: 'ボタンが非活性（押せない状態）になっていること'},
                {actor: '管理者', action: '（フロントエンドの制御を迂回してAPIを直接叩いた場合）POST /api/teams/balance', check: 'HTTP 400 エラーが返却され、エラーメッセージがトーストで表示されること'},
              ]" :key="idx" class="flex gap-3 text-xs">
                <div class="flex-none flex flex-col items-center">
                  <div class="w-6 h-6 rounded-full bg-amber-500 text-white flex items-center justify-center text-[10px] font-bold">{{ idx + 1 }}</div>
                  <div v-if="idx < 1" class="w-px flex-1 bg-amber-200 my-1" />
                </div>
                <div class="pb-3 flex-1">
                  <div class="flex items-center gap-2 mb-1">
                    <span class="bg-amber-200 text-amber-800 font-bold px-1.5 py-0.5 text-[10px]">{{ step.actor }}</span>
                    <span class="text-neutral-800 font-medium">{{ step.action }}</span>
                  </div>
                  <div class="flex gap-1 items-start text-[10px] text-neutral-500">
                    <span class="text-amber-500 font-bold shrink-0">確認:</span>
                    <span>{{ step.check }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

        </div>
      </section>

      <!-- テストケース一覧 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-violet-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">3. テストケース一覧</h2>
        </div>
        <div class="p-5">
          <table class="w-full text-xs border-collapse">
            <thead>
              <tr class="bg-neutral-100 text-neutral-600">
                <th class="border border-neutral-200 px-3 py-2 text-left w-20">テストID</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">確認ポイント</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="tc in [
                ['ST-01', '6人3チームの基本フロー全体', 'チーム分け実行〜ユーザー反映まで一連の流れが動作すること'],
                ['ST-02', '補正値調整後のチーム分け', '補正値が反映されたスコアでチームが構成されること'],
                ['ST-03', '同名プレイヤーの重複名解決', 'チーム結果に(1)(2)が付いて区別されること'],
                ['ST-04', '0人チーム分けの防御', 'ボタン非活性およびAPIエラーが正しく動作すること'],
                ['ST-05', '設計書へのアクセス', '管理者画面の「設計書」ボタンから設計書一覧に遷移できること'],
                ['ST-06', '設計書のアクセス制限確認', '設計書がユーザー画面からはリンクされていないこと'],
                ['ST-07', '警告表示の確認', '参加者数がチーム数より少ない場合に警告が表示されること'],
                ['ST-08', '自分の★表示', 'ユーザー自身が自分のプレイヤーに★が表示されること'],
              ]" :key="tc[0]" class="hover:bg-neutral-50">
                <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                <td class="border border-neutral-200 px-3 py-2 font-medium">{{ tc[1] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ tc[2] }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

    </div>
  </div>
</template>
