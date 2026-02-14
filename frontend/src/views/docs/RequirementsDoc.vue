<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
  <div class="min-h-screen bg-neutral-100">
    <!-- ヘッダー -->
    <header class="bg-blue-700 text-white px-6 py-4 flex items-center gap-4 shadow">
      <button @click="router.push('/owner/docs')" class="text-blue-200 hover:text-white transition-colors text-sm">← 設計書一覧に戻る</button>
      <div class="w-px h-4 bg-blue-500" />
      <h1 class="text-base font-bold">📋 要件定義書</h1>
      <div class="flex-1" />
      <span class="text-xs text-blue-200">バージョン 1.0 ／ 2025年作成</span>
    </header>

    <div class="max-w-4xl mx-auto px-6 py-8 space-y-8">

      <!-- 1. 目的と背景 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">1. このシステムを作った目的と背景</h2>
        </div>
        <div class="p-5 space-y-4">
          <div class="p-4 bg-blue-50 border-l-4 border-blue-400">
            <p class="text-sm font-semibold text-blue-800 mb-1">📌 解決したかった問題</p>
            <p class="text-sm text-neutral-700">
              Apex Legends のカスタムマッチで複数チームに分かれて遊ぶとき、<strong>「なんとなく」でチームを決めると強いプレイヤーが偏ってしまい、試合が一方的になってしまう</strong> という問題がありました。
            </p>
          </div>
          <div class="p-4 bg-emerald-50 border-l-4 border-emerald-400">
            <p class="text-sm font-semibold text-emerald-800 mb-1">✅ このシステムで実現すること</p>
            <p class="text-sm text-neutral-700">
              参加者のゲーム内ランクと個人差を考慮して、<strong>自動的に公平なチームを組み分ける</strong> ことで、全員が楽しめるバランスのよい試合環境を提供します。
            </p>
          </div>
        </div>
      </section>

      <!-- 2. 利用者の種類 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">2. 利用者の種類（ユーザー種別）</h2>
        </div>
        <div class="p-5">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <!-- オーナー -->
            <div class="border-2 border-neutral-900 p-4">
              <div class="flex items-center gap-2 mb-3">
                <div class="w-8 h-8 bg-neutral-900 text-white flex items-center justify-center text-sm font-bold">管</div>
                <div>
                  <p class="font-bold text-sm">オーナー（管理者）</p>
                  <p class="text-xs text-neutral-500">カスタムマッチを主催する人</p>
                </div>
              </div>
              <ul class="text-xs text-neutral-700 space-y-1.5">
                <li class="flex gap-2"><span class="text-emerald-500 font-bold">✓</span>参加者の追加・削除・編集</li>
                <li class="flex gap-2"><span class="text-emerald-500 font-bold">✓</span>戦闘力補正値の調整（個人差の反映）</li>
                <li class="flex gap-2"><span class="text-emerald-500 font-bold">✓</span>チーム数の設定（2〜6チーム）</li>
                <li class="flex gap-2"><span class="text-emerald-500 font-bold">✓</span>チーム分けの実行・解散</li>
                <li class="flex gap-2"><span class="text-emerald-500 font-bold">✓</span>設計書の閲覧</li>
              </ul>
              <div class="mt-3 text-xs text-neutral-500 bg-neutral-100 p-2">アクセスURL: <code>/owner</code></div>
            </div>
            <!-- ユーザー -->
            <div class="border-2 border-blue-400 p-4">
              <div class="flex items-center gap-2 mb-3">
                <div class="w-8 h-8 bg-blue-500 text-white flex items-center justify-center text-sm font-bold">参</div>
                <div>
                  <p class="font-bold text-sm">一般ユーザー（参加者）</p>
                  <p class="text-xs text-neutral-500">カスタムマッチに参加するプレイヤー</p>
                </div>
              </div>
              <ul class="text-xs text-neutral-700 space-y-1.5">
                <li class="flex gap-2"><span class="text-emerald-500 font-bold">✓</span>自分の名前・ランクを登録</li>
                <li class="flex gap-2"><span class="text-emerald-500 font-bold">✓</span>参加者一覧を閲覧</li>
                <li class="flex gap-2"><span class="text-emerald-500 font-bold">✓</span>チーム分け結果を閲覧（★で自分を識別）</li>
                <li class="flex gap-2"><span class="text-neutral-400 font-bold">✗</span>チーム分けの実行はできない</li>
                <li class="flex gap-2"><span class="text-neutral-400 font-bold">✗</span>他人のデータを編集できない</li>
              </ul>
              <div class="mt-3 text-xs text-neutral-500 bg-neutral-100 p-2">アクセスURL: <code>/</code>（トップページ）</div>
            </div>
          </div>
        </div>
      </section>

      <!-- 3. 機能要件 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">3. 機能要件（システムが「できること」の一覧）</h2>
        </div>
        <div class="p-5">
          <p class="text-xs text-neutral-500 mb-4">※ 優先度：◎＝必須、○＝重要、△＝あると便利</p>
          <table class="w-full text-sm border-collapse">
            <thead>
              <tr class="bg-neutral-100 text-xs text-neutral-600">
                <th class="border border-neutral-200 px-3 py-2 text-left w-8">No.</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">機能名</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">内容</th>
                <th class="border border-neutral-200 px-3 py-2 text-center w-20">使える人</th>
                <th class="border border-neutral-200 px-3 py-2 text-center w-16">優先度</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(row, i) in [
                ['1', 'プレイヤー登録', '名前とランクを入力して参加登録する', '全員', '◎'],
                ['2', 'プレイヤー編集', '登録済みの名前・ランクを変更する', '全員*', '◎'],
                ['3', 'プレイヤー削除', '参加者を一覧から削除する', '管理者', '◎'],
                ['4', '戦闘力補正', '同ランクでも個人差を±1ずつ調整する（−3〜+3）', '管理者', '○'],
                ['5', 'チーム数設定', '分けるチームの数を2〜6で選択する', '管理者', '◎'],
                ['6', 'チーム分け実行', 'アルゴリズムで公平にチームを自動編成する', '管理者', '◎'],
                ['7', 'チーム解散', '現在のチーム分け結果をリセットする', '管理者', '○'],
                ['8', '重複名解決', '同じ名前の人がいたとき自動で区別する（名前(1), 名前(2)）', '自動', '○'],
                ['9', 'リアルタイム同期', '3秒ごとに自動でサーバーと同期し、全員の画面を最新状態に保つ', '全員', '○'],
                ['10', '自分の識別', 'チーム結果で自分のプレイヤーに★マークを表示', '一般ユーザー', '△'],
              ]" :key="i" class="hover:bg-neutral-50">
                <td class="border border-neutral-200 px-3 py-2 text-neutral-500 text-xs">{{ row[0] }}</td>
                <td class="border border-neutral-200 px-3 py-2 font-medium">{{ row[1] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-xs text-neutral-700">{{ row[2] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-center text-xs">{{ row[3] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-center font-bold"
                    :class="row[4]==='◎' ? 'text-red-600' : row[4]==='○' ? 'text-amber-600' : 'text-neutral-400'">{{ row[4] }}</td>
              </tr>
            </tbody>
          </table>
          <p class="text-xs text-neutral-400 mt-2">* 一般ユーザーは自分自身の情報のみ編集可能</p>
        </div>
      </section>

      <!-- 4. 非機能要件 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">4. 非機能要件（「どう動くか」の条件）</h2>
        </div>
        <div class="p-5 space-y-3">
          <div v-for="item in [
            { icon: '⚡', label: 'パフォーマンス', content: 'チーム分け実行・プレイヤー追加などの操作は 1 秒以内に応答すること' },
            { icon: '🔄', label: 'リアルタイム性', content: '管理者がチーム分けを実行してから 3 秒以内に一般ユーザー画面に反映されること（ポーリング方式）' },
            { icon: '📱', label: '対応デバイス', content: 'PC・スマートフォン・タブレットのWebブラウザから利用できること（レスポンシブデザイン）' },
            { icon: '🛡️', label: 'データ保護', content: '設計書は管理者専用URL（/owner/docs）からのみアクセス可能とする' },
            { icon: '🔧', label: '保守性', content: 'ランクのスコアや補正値の範囲を、コードの1箇所を変更するだけで調整できること' },
          ]" :key="item.label" class="flex gap-3 p-3 border border-neutral-100 bg-neutral-50">
            <span class="text-xl leading-none">{{ item.icon }}</span>
            <div>
              <p class="text-xs font-bold text-neutral-800 mb-0.5">{{ item.label }}</p>
              <p class="text-xs text-neutral-600">{{ item.content }}</p>
            </div>
          </div>
        </div>
      </section>

      <!-- 5. 制約条件 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">5. 制約条件（システムの制限事項）</h2>
        </div>
        <div class="p-5">
          <div class="space-y-2">
            <div v-for="c in [
              ['データの永続化なし', 'サーバーを再起動するとプレイヤーデータ・チーム情報はすべてリセットされます。同日内のカスタムマッチ利用を想定しています。'],
              ['認証機能なし', '管理者画面（/owner）はURLを知っている人であれば誰でもアクセスできます。URLは参加者に広めないよう注意してください。'],
              ['チーム数の上限', '作成できるチームは最大6チームです。'],
              ['プレイヤー名の文字数', 'プレイヤー名は1文字以上30文字以内で登録してください。'],
              ['補正値の範囲', '戦闘力補正値は−3〜+3の整数のみ設定できます。'],
              ['無料プランの制限', '本番環境(Render.com)では15分間アクセスがないとサーバーがスリープします。次回アクセス時に30秒〜1分の起動時間がかかります。'],
            ]" :key="c[0]" class="flex gap-3 p-3 border border-amber-100 bg-amber-50">
              <span class="text-amber-500 font-bold text-sm shrink-0 mt-0.5">⚠</span>
              <div>
                <p class="text-xs font-bold text-neutral-800">{{ c[0] }}</p>
                <p class="text-xs text-neutral-600 mt-0.5">{{ c[1] }}</p>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- 6. 用語定義 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">6. 用語定義</h2>
        </div>
        <div class="p-5">
          <table class="w-full text-sm border-collapse">
            <thead>
              <tr class="bg-neutral-100 text-xs text-neutral-600">
                <th class="border border-neutral-200 px-3 py-2 text-left w-32">用語</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">説明</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="term in [
                ['ランク', 'Apex Legends のゲーム内ランク。ブロンズ〜プレデターの7段階。実力の目安として使用する。'],
                ['基本スコア', 'ランクに対応する数値（ブロンズ=10、シルバー=20、…、プレデター=70）。チームの戦力比較に使用する。'],
                ['戦闘力補正値（オフセット）', '同じランクでも個人差がある場合に管理者が調整できる値（−3〜+3）。基本スコアに加算される。'],
                ['実効スコア（effective_score）', '基本スコア＋補正値の合計。実際のチーム分けアルゴリズムで使用される戦力値。最低値は1。'],
                ['チーム分けアルゴリズム', '30回のランダムシャッフルを試み、チーム間のスコア差が最も小さいチーム分けを採用する仕組み。'],
                ['重複名解決', '同じ名前の参加者が複数いるとき、「名前(1)」「名前(2)」と自動で区別する機能。'],
                ['ポーリング', '3秒ごとに自動でサーバーに問い合わせて最新情報を取得する仕組み。'],
              ]" :key="term[0]" class="hover:bg-neutral-50">
                <td class="border border-neutral-200 px-3 py-2 font-medium text-blue-700 text-xs">{{ term[0] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-xs text-neutral-700">{{ term[1] }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>
    </div>
  </div>
</template>
