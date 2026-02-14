<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
  <div class="min-h-screen bg-neutral-100">
    <header class="bg-emerald-700 text-white px-6 py-4 flex items-center gap-4 shadow">
      <button @click="router.push('/owner/docs')" class="text-emerald-200 hover:text-white transition-colors text-sm">← 設計書一覧に戻る</button>
      <div class="w-px h-4 bg-emerald-500" />
      <h1 class="text-base font-bold">🧪 単体テスト仕様書</h1>
      <div class="flex-1" />
      <span class="text-xs text-emerald-200">バージョン 1.0 ／ 2025年作成</span>
    </header>

    <div class="max-w-4xl mx-auto px-6 py-8 space-y-8">

      <!-- テスト方針 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-emerald-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">1. テスト方針</h2>
        </div>
        <div class="p-5 space-y-3">
          <div class="p-4 bg-emerald-50 border-l-4 border-emerald-400">
            <p class="text-sm font-semibold text-emerald-800 mb-1">🎯 単体テストとは？</p>
            <p class="text-sm text-neutral-700">
              「部品」を単独でテストすることです。レストランに例えると、「料理を提供する前に食材や調味料を個別に確認する」イメージです。
              プログラムの各関数やロジックが、期待通りに動くかを個別に検証します。
            </p>
          </div>
          <div class="grid grid-cols-2 gap-3 text-xs">
            <div class="border border-neutral-200 p-3 bg-neutral-50">
              <p class="font-bold text-neutral-700 mb-1">テスト対象</p>
              <ul class="space-y-1 text-neutral-600">
                <li>• バックエンド：Rustの関数・ロジック</li>
                <li>• 実効スコア計算（effective_score）</li>
                <li>• チーム分けアルゴリズム（balance_teams）</li>
                <li>• 重複名解決ロジック（resolve_duplicate_names）</li>
              </ul>
            </div>
            <div class="border border-neutral-200 p-3 bg-neutral-50">
              <p class="font-bold text-neutral-700 mb-1">テスト実行方法</p>
              <p class="text-neutral-600">Rustの標準テストフレームワーク（<code class="bg-neutral-200 px-1">cargo test</code>）を使用。<br>ソースコード内の <code class="bg-neutral-200 px-1">#[cfg(test)]</code> ブロックに記述。</p>
            </div>
          </div>
        </div>
      </section>

      <!-- テストケース一覧 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-emerald-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">2. テストケース一覧</h2>
        </div>
        <div class="p-5 space-y-6">

          <!-- UT-01: effective_score -->
          <div>
            <div class="flex items-center gap-2 mb-3">
              <span class="bg-emerald-600 text-white text-xs font-bold px-2 py-0.5">UT-01</span>
              <span class="text-sm font-bold text-neutral-800">実効スコア計算 effective_score()</span>
            </div>
            <table class="w-full text-xs border-collapse">
              <thead>
                <tr class="bg-neutral-100 text-neutral-600">
                  <th class="border border-neutral-200 px-3 py-2 text-left w-20">テストID</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容（何を確認するか）</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">入力条件</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">期待される結果</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="tc in [
                  ['UT-01-01', '補正値ありの実効スコアが正しく計算されること', 'ランク:ゴールド(30)、補正値:+2', '実効スコア = 32'],
                  ['UT-01-02', '補正値マイナスの実効スコアが正しく計算されること', 'ランク:ブロンズ(10)、補正値:−3', '実効スコア = 7'],
                  ['UT-01-03', '実効スコアが極端にマイナスでも最低値1になること', 'ランク:ブロンズ(10)、補正値:−10', '実効スコア = 1（clamp）'],
                  ['UT-01-04', '補正値0のとき基本スコアそのままであること', 'ランク:プレデター(70)、補正値:0', '実効スコア = 70'],
                ]" :key="tc[0]" class="hover:bg-neutral-50">
                  <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                  <td class="border border-neutral-200 px-3 py-2">{{ tc[1] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ tc[2] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-emerald-700 font-medium">{{ tc[3] }}</td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- UT-02: balance_teams -->
          <div>
            <div class="flex items-center gap-2 mb-3">
              <span class="bg-emerald-600 text-white text-xs font-bold px-2 py-0.5">UT-02</span>
              <span class="text-sm font-bold text-neutral-800">チーム分けアルゴリズム balance_teams()</span>
            </div>
            <table class="w-full text-xs border-collapse">
              <thead>
                <tr class="bg-neutral-100 text-neutral-600">
                  <th class="border border-neutral-200 px-3 py-2 text-left w-20">テストID</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">入力条件</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">期待される結果</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="tc in [
                  ['UT-02-01', 'すべての参加者がいずれかのチームに割り当てられること', '4人の参加者、2チーム', '全チームの人数合計 = 4人'],
                  ['UT-02-02', '指定したチーム数のチームが作成されること', '4人の参加者、2チーム', 'teams.len() = 2'],
                  ['UT-02-03', '参加者が0人でも空チームが返ること（エラーにならないこと）', '0人、2チーム', '空のチーム2つが返る'],
                  ['UT-02-04', 'チーム数が参加者数より多い場合、参加者数がチーム数になること', '2人の参加者、5チーム', 'teams.len() = 2'],
                ]" :key="tc[0]" class="hover:bg-neutral-50">
                  <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                  <td class="border border-neutral-200 px-3 py-2">{{ tc[1] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ tc[2] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-emerald-700 font-medium">{{ tc[3] }}</td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- UT-03: resolve_duplicate_names -->
          <div>
            <div class="flex items-center gap-2 mb-3">
              <span class="bg-emerald-600 text-white text-xs font-bold px-2 py-0.5">UT-03</span>
              <span class="text-sm font-bold text-neutral-800">重複名解決 resolve_duplicate_names()</span>
            </div>
            <table class="w-full text-xs border-collapse">
              <thead>
                <tr class="bg-neutral-100 text-neutral-600">
                  <th class="border border-neutral-200 px-3 py-2 text-left w-20">テストID</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">入力条件</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">期待される結果</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="tc in [
                  ['UT-03-01', '同名プレイヤーに連番付きdisplay_nameが設定されること', '「あかし」が2人登録', '1人目:あかし(1)、2人目:あかし(2)'],
                  ['UT-03-02', '重複しない名前のdisplay_nameはnullのままであること', '全員異なる名前', '全員のdisplay_name = null'],
                  ['UT-03-03', '3人重複でも正しく連番が付くこと', '「たろう」が3人登録', 'たろう(1)、たろう(2)、たろう(3)'],
                ]" :key="tc[0]" class="hover:bg-neutral-50">
                  <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                  <td class="border border-neutral-200 px-3 py-2">{{ tc[1] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ tc[2] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-emerald-700 font-medium">{{ tc[3] }}</td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- UT-04: バリデーション -->
          <div>
            <div class="flex items-center gap-2 mb-3">
              <span class="bg-emerald-600 text-white text-xs font-bold px-2 py-0.5">UT-04</span>
              <span class="text-sm font-bold text-neutral-800">プレイヤー名バリデーション</span>
            </div>
            <table class="w-full text-xs border-collapse">
              <thead>
                <tr class="bg-neutral-100 text-neutral-600">
                  <th class="border border-neutral-200 px-3 py-2 text-left w-20">テストID</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">テスト内容</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">入力条件</th>
                  <th class="border border-neutral-200 px-3 py-2 text-left">期待される結果</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="tc in [
                  ['UT-04-01', '空の名前はエラーになること', '名前: \"\"（空文字）', 'HTTP 400 / エラーメッセージ返却'],
                  ['UT-04-02', 'スペースのみの名前はエラーになること', '名前: \"   \"（スペースのみ）', 'HTTP 400 / trim後は空文字のためエラー'],
                  ['UT-04-03', '30文字の名前は登録できること', '30文字のプレイヤー名', 'HTTP 201 / 正常登録'],
                  ['UT-04-04', '31文字以上の名前はエラーになること', '31文字のプレイヤー名', 'HTTP 400 / エラーメッセージ返却'],
                ]" :key="tc[0]" class="hover:bg-neutral-50">
                  <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ tc[0] }}</td>
                  <td class="border border-neutral-200 px-3 py-2">{{ tc[1] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ tc[2] }}</td>
                  <td class="border border-neutral-200 px-3 py-2 text-emerald-700 font-medium">{{ tc[3] }}</td>
                </tr>
              </tbody>
            </table>
          </div>

        </div>
      </section>

    </div>
  </div>
</template>
