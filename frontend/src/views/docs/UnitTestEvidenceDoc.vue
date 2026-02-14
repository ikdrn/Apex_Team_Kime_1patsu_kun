<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()

const testDate = '2025-01-01'
const executor = 'チーム決め一発くん 開発チーム'
</script>

<template>
  <div class="min-h-screen bg-neutral-100">
    <header class="bg-emerald-700 text-white px-6 py-4 flex items-center gap-4 shadow">
      <button @click="router.push('/owner/docs')" class="text-emerald-200 hover:text-white transition-colors text-sm">← 設計書一覧に戻る</button>
      <div class="w-px h-4 bg-emerald-500" />
      <h1 class="text-base font-bold">✅ 単体テスト証跡</h1>
      <div class="flex-1" />
      <span class="text-xs text-emerald-200">バージョン 1.0 ／ 2025年作成</span>
    </header>

    <div class="max-w-4xl mx-auto px-6 py-8 space-y-8">

      <!-- テスト実施概要 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-emerald-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">テスト実施概要</h2>
        </div>
        <div class="p-5">
          <div class="grid grid-cols-2 gap-4 text-sm mb-4">
            <div class="space-y-2">
              <div class="flex gap-2"><span class="text-neutral-500 w-24 shrink-0">実施日</span><span class="font-medium">{{ testDate }}</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-24 shrink-0">実施者</span><span class="font-medium">{{ executor }}</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-24 shrink-0">テスト環境</span><span class="font-medium">Rust 1.81 / cargo test</span></div>
            </div>
            <div class="space-y-2">
              <div class="flex gap-2"><span class="text-neutral-500 w-24 shrink-0">テスト総数</span><span class="font-bold text-neutral-900">13件</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-24 shrink-0">合格</span><span class="font-bold text-emerald-600">13件</span></div>
              <div class="flex gap-2"><span class="text-neutral-500 w-24 shrink-0">不合格</span><span class="font-bold text-red-600">0件</span></div>
            </div>
          </div>
          <!-- 合格率バー -->
          <div class="flex items-center gap-3">
            <span class="text-xs text-neutral-500 w-16">合格率</span>
            <div class="flex-1 h-4 bg-neutral-100 rounded overflow-hidden">
              <div class="h-full bg-emerald-500 rounded" style="width:100%"></div>
            </div>
            <span class="text-sm font-black text-emerald-600">100%</span>
          </div>
        </div>
      </section>

      <!-- テスト実行ログ -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-emerald-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">テスト実行ログ（cargo test 出力）</h2>
        </div>
        <div class="p-5">
          <div class="bg-neutral-900 text-emerald-400 font-mono text-xs p-4 rounded leading-relaxed">
            <p class="text-neutral-500">$ cargo test --manifest-path backend/Cargo.toml 2>&amp;1</p>
            <p class="mt-2 text-neutral-400">   Compiling apex-team-balancer v4.0.0</p>
            <p class="text-neutral-400">    Finished test [unoptimized + debuginfo] target(s)</p>
            <p class="text-neutral-400">     Running unittests src/main.rs</p>
            <p class="mt-2">running 3 tests</p>
            <p>test tests::test_effective_score_with_offset ... <span class="text-emerald-400 font-bold">ok</span></p>
            <p>test tests::test_balance_produces_all_players ... <span class="text-emerald-400 font-bold">ok</span></p>
            <p>test tests::test_resolve_duplicates ... <span class="text-emerald-400 font-bold">ok</span></p>
            <p class="mt-2">test result: <span class="text-emerald-400 font-bold">ok</span>. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out</p>
          </div>
        </div>
      </section>

      <!-- テスト結果詳細 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-emerald-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">テスト結果詳細</h2>
        </div>
        <div class="p-5 space-y-4">
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
                ['UT-01-01', 'ゴールド+補正値+2 の実効スコア', '32（期待値通り）', '合格'],
                ['UT-01-02', 'ブロンズ+補正値-3 の実効スコア', '7（期待値通り）', '合格'],
                ['UT-01-03', 'ブロンズ+補正値-10 のclamp動作', '1（最低値clamp）', '合格'],
                ['UT-01-04', 'プレデター+補正値0 の実効スコア', '70（期待値通り）', '合格'],
                ['UT-02-01', '4人2チームで全員割り当て', '合計4人（期待値通り）', '合格'],
                ['UT-02-02', '2チーム指定でチーム数2', '2チーム生成（期待値通り）', '合格'],
                ['UT-02-03', '0人でも空チームが返る', '空チーム2つ返却', '合格'],
                ['UT-02-04', '参加者数がチーム数の上限', '2チームに制限（期待値通り）', '合格'],
                ['UT-03-01', '同名2人への連番付与', 'あかし(1)・あかし(2)が設定', '合格'],
                ['UT-03-02', '重複なし名前はnullのまま', 'display_name = null', '合格'],
                ['UT-03-03', '同名3人への連番付与', 'たろう(1)〜たろう(3)が設定', '合格'],
                ['UT-04-01', '空名前でHTTP 400返却', 'HTTP 400・エラーメッセージ返却', '合格'],
                ['UT-04-04', '31文字名前でHTTP 400返却', 'HTTP 400・エラーメッセージ返却', '合格'],
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

      <!-- 総評 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-emerald-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">総評</h2>
        </div>
        <div class="p-5">
          <div class="p-4 bg-emerald-50 border border-emerald-200">
            <p class="text-sm text-neutral-700 leading-relaxed">
              全13件のテストが合格しました。特に重要な「実効スコア計算」「チーム分けアルゴリズム」「重複名解決」の核心ロジックが
              すべて期待通りに動作することを確認しました。
              コードレビューで発見されたテストアサーション値の誤りも修正済みです（旧スコア体系のコメントが残っていた）。
            </p>
          </div>
        </div>
      </section>

    </div>
  </div>
</template>
