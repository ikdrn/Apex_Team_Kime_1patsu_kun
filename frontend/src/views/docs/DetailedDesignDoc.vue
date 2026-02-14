<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
  <div class="min-h-screen bg-neutral-100">
    <header class="bg-blue-700 text-white px-6 py-4 flex items-center gap-4 shadow">
      <button @click="router.push('/owner/docs')" class="text-blue-200 hover:text-white transition-colors text-sm">← 設計書一覧に戻る</button>
      <div class="w-px h-4 bg-blue-500" />
      <h1 class="text-base font-bold">⚙️ 詳細設計書</h1>
      <div class="flex-1" />
      <span class="text-xs text-blue-200">バージョン 1.0 ／ 2025年作成</span>
    </header>

    <div class="max-w-4xl mx-auto px-6 py-8 space-y-8">

      <!-- 1. チーム分けアルゴリズム詳細 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">1. チーム分けアルゴリズムの詳細（どうやって公平に分けるか）</h2>
        </div>
        <div class="p-5 space-y-4">
          <p class="text-xs text-neutral-600 leading-relaxed">
            「完全に公平なチーム分け」は計算が非常に難しい問題です（NP困難問題）。そこで本システムでは、
            <strong>「ランダムに30回試して、一番バランスの良い結果を採用する」</strong> という方式を使っています。
          </p>

          <!-- フロー図 -->
          <div class="border border-neutral-200 bg-neutral-50 p-4">
            <p class="text-xs font-bold text-neutral-700 mb-4 text-center">チーム分けアルゴリズムのフロー</p>
            <div class="flex flex-col items-center gap-1">

              <div class="border-2 border-blue-400 bg-blue-50 px-8 py-2 text-xs font-bold text-blue-800">開始</div>
              <div class="w-px h-3 bg-neutral-400" />

              <div class="border border-neutral-300 bg-white px-6 py-2 text-xs text-center max-w-sm">
                <strong>①</strong> 参加者全員をランダムな順番に並べ替える（シャッフル）
              </div>
              <div class="w-px h-3 bg-neutral-400" />

              <div class="border border-neutral-300 bg-white px-6 py-2 text-xs text-center max-w-sm">
                <strong>②</strong> 実効スコアの高い順に並べ替える（同スコアはランダム順を維持）
              </div>
              <div class="w-px h-3 bg-neutral-400" />

              <div class="border border-neutral-300 bg-white px-6 py-2 text-xs text-center max-w-sm">
                <strong>③</strong> 強い人から順番に、その時点で最もスコアが低いチームに割り当てる（貪欲法）
              </div>
              <div class="w-px h-3 bg-neutral-400" />

              <div class="border border-neutral-300 bg-white px-6 py-2 text-xs text-center max-w-sm">
                <strong>④</strong> チーム間のスコア差（最大 − 最小）を計算する
              </div>
              <div class="w-px h-3 bg-neutral-400" />

              <div class="border border-amber-300 bg-amber-50 px-6 py-2 text-xs text-center max-w-sm">
                <strong>⑤</strong> 今回のスコア差 ＜ これまでの最小スコア差？
              </div>

              <div class="flex gap-8">
                <div class="flex flex-col items-center">
                  <div class="text-xs text-emerald-600 font-bold">YES</div>
                  <div class="w-px h-3 bg-neutral-400" />
                  <div class="border border-emerald-300 bg-emerald-50 px-4 py-1.5 text-[10px] text-center">今回の結果を「最良」として保存</div>
                </div>
                <div class="flex flex-col items-center">
                  <div class="text-xs text-neutral-400 font-bold">NO</div>
                  <div class="w-px h-3 bg-neutral-400" />
                  <div class="border border-neutral-200 bg-white px-4 py-1.5 text-[10px] text-center">前の「最良」を維持</div>
                </div>
              </div>

              <div class="w-px h-3 bg-neutral-400" />
              <div class="border border-amber-300 bg-amber-50 px-6 py-2 text-xs text-center max-w-sm">
                <strong>⑥</strong> 30回繰り返したか？
              </div>
              <div class="flex gap-8">
                <div class="flex flex-col items-center">
                  <div class="text-xs text-neutral-400 font-bold">NO → ①に戻る</div>
                </div>
                <div class="flex flex-col items-center">
                  <div class="text-xs text-emerald-600 font-bold">YES</div>
                  <div class="w-px h-3 bg-neutral-400" />
                  <div class="border-2 border-emerald-400 bg-emerald-50 px-8 py-2 text-xs font-bold text-emerald-800">最良のチーム分け結果を返す</div>
                </div>
              </div>
            </div>
          </div>

          <!-- 具体例 -->
          <div class="border border-neutral-200 bg-white p-4">
            <p class="text-xs font-bold text-neutral-700 mb-3">📌 具体例：4人を2チームに分ける場合</p>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <p class="text-[10px] font-bold text-neutral-500 mb-2">参加者</p>
                <div class="space-y-1">
                  <div v-for="p in [
                    {name:'Aさん', rank:'プレデター', score:70},
                    {name:'Bさん', rank:'マスター', score:60},
                    {name:'Cさん', rank:'ゴールド', score:30},
                    {name:'Dさん', rank:'ブロンズ', score:10},
                  ]" :key="p.name" class="flex items-center gap-2 text-[10px]">
                    <span class="font-bold w-8">{{ p.name }}</span>
                    <span class="text-neutral-500">{{ p.rank }}</span>
                    <span class="font-mono font-bold ml-auto">{{ p.score }}</span>
                  </div>
                </div>
              </div>
              <div>
                <p class="text-[10px] font-bold text-neutral-500 mb-2">最良の結果</p>
                <div class="space-y-2">
                  <div class="border border-blue-200 bg-blue-50 p-2">
                    <p class="text-[10px] font-bold text-blue-700">TEAM 1</p>
                    <p class="text-[10px]">A(70) + D(10) = <strong>80</strong></p>
                  </div>
                  <div class="border border-orange-200 bg-orange-50 p-2">
                    <p class="text-[10px] font-bold text-orange-700">TEAM 2</p>
                    <p class="text-[10px]">B(60) + C(30) = <strong>90</strong></p>
                  </div>
                  <p class="text-[10px] text-emerald-600 font-bold">スコア差: 10（均衡）</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- 2. 戦闘力補正のしくみ -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">2. 戦闘力補正（スコアオフセット）のしくみ</h2>
        </div>
        <div class="p-5 space-y-4">
          <div class="p-3 bg-neutral-50 border border-neutral-200 text-xs text-neutral-700 leading-relaxed">
            <strong>「実効スコア」= 基本スコア + 補正値</strong>（最低値は1）<br>
            同じランクでも「このプレイヤーは明らかに強い/弱い」という場合に、管理者が±1ずつ調整することで、より公平なチーム分けが実現できます。
          </div>

          <table class="w-full text-xs border-collapse">
            <thead>
              <tr class="bg-neutral-100 text-neutral-600">
                <th class="border border-neutral-200 px-3 py-2 text-left">ランク</th>
                <th class="border border-neutral-200 px-3 py-2 text-center">基本スコア</th>
                <th class="border border-neutral-200 px-3 py-2 text-center">補正 −3</th>
                <th class="border border-neutral-200 px-3 py-2 text-center">補正 0（デフォルト）</th>
                <th class="border border-neutral-200 px-3 py-2 text-center">補正 +3</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="r in [
                ['ブロンズ', 10, 7, 10, 13],
                ['シルバー', 20, 17, 20, 23],
                ['ゴールド', 30, 27, 30, 33],
                ['プラチナ', 40, 37, 40, 43],
                ['ダイヤモンド', 50, 47, 50, 53],
                ['マスター', 60, 57, 60, 63],
                ['プレデター', 70, 67, 70, 73],
              ]" :key="r[0]" class="hover:bg-neutral-50">
                <td class="border border-neutral-200 px-3 py-2 font-medium">{{ r[0] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-center font-mono">{{ r[1] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-center font-mono text-red-500">{{ r[2] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-center font-mono font-bold bg-neutral-50">{{ r[3] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-center font-mono text-blue-500">{{ r[4] }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      <!-- 3. 重複名解決ロジック -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">3. 重複名解決ロジック（同じ名前の人がいたときの処理）</h2>
        </div>
        <div class="p-5 space-y-3">
          <p class="text-xs text-neutral-600">チーム分け実行時に、同じ名前のプレイヤーが複数いる場合、自動で区別できる名前を付けます。</p>
          <div class="grid grid-cols-2 gap-4">
            <div class="border border-neutral-200 p-3 bg-neutral-50">
              <p class="text-xs font-bold text-neutral-700 mb-2">処理前（登録データ）</p>
              <div class="space-y-1 text-xs font-mono">
                <div class="flex gap-2"><span class="text-neutral-400">ID-001</span><span>あかし</span></div>
                <div class="flex gap-2"><span class="text-neutral-400">ID-002</span><span>いのうえ</span></div>
                <div class="flex gap-2"><span class="text-neutral-400">ID-003</span><span>あかし</span></div>
              </div>
            </div>
            <div class="border border-emerald-200 p-3 bg-emerald-50">
              <p class="text-xs font-bold text-emerald-700 mb-2">処理後（display_name）</p>
              <div class="space-y-1 text-xs font-mono">
                <div class="flex gap-2"><span class="text-neutral-400">ID-001</span><span class="font-bold">あかし(1)</span></div>
                <div class="flex gap-2"><span class="text-neutral-400">ID-002</span><span>いのうえ（変更なし）</span></div>
                <div class="flex gap-2"><span class="text-neutral-400">ID-003</span><span class="font-bold">あかし(2)</span></div>
              </div>
            </div>
          </div>
          <p class="text-xs text-neutral-500 bg-amber-50 border border-amber-200 p-2">
            ⚠ 重複解決はチーム分け実行時のみ行われます。登録画面では表示されません。
          </p>
        </div>
      </section>

      <!-- 4. リアルタイム同期 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">4. リアルタイム同期の仕組み（ポーリング方式）</h2>
        </div>
        <div class="p-5 space-y-4">
          <div class="grid grid-cols-3 gap-3 text-center text-xs">
            <div class="border border-blue-200 bg-blue-50 p-3">
              <p class="font-bold text-blue-700 mb-1">①画面表示</p>
              <p class="text-neutral-600">ページを開くと初期データを取得し、3秒ポーリングを開始</p>
            </div>
            <div class="border border-neutral-200 bg-neutral-50 p-3">
              <p class="font-bold text-neutral-700 mb-1">②3秒ごとに自動取得</p>
              <p class="text-neutral-600">プレイヤー一覧 / チーム分け結果 / 設定（チーム数）を定期的にサーバーから取得。バックグラウンド動作のためローディング表示は出ない。</p>
            </div>
            <div class="border border-emerald-200 bg-emerald-50 p-3">
              <p class="font-bold text-emerald-700 mb-1">③ページを閉じると停止</p>
              <p class="text-neutral-600">ポーリングを停止してサーバーへの無駄なリクエストを防ぐ</p>
            </div>
          </div>
          <div class="p-3 bg-neutral-50 border border-neutral-200 text-xs text-neutral-700">
            <strong>技術的な補足：</strong> WebSocket（常時接続）ではなくポーリング（定期的な問い合わせ）方式を採用。
            シンプルな実装で同等の体験を実現しています。管理者と一般ユーザーで同じ仕組みを使用。
          </div>
        </div>
      </section>

      <!-- 5. バリデーション仕様 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">5. 入力バリデーション仕様（入力チェックのルール）</h2>
        </div>
        <div class="p-5">
          <table class="w-full text-xs border-collapse">
            <thead>
              <tr class="bg-neutral-100 text-neutral-600">
                <th class="border border-neutral-200 px-3 py-2 text-left">項目</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">チェック場所</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">ルール</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">NGの場合</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="v in [
                ['プレイヤー名', 'フロント + サーバー', '1文字以上・30文字以内・前後の空白は自動削除', 'ボタン非活性 / エラーレスポンス'],
                ['ランク', 'フロント（選択肢固定）', '7種のランクから必ず1つ選択', '選択肢以外は送信不可'],
                ['補正値delta', 'サーバー', '+1 または -1 のみ（-1〜+1にclamp）', '範囲外は自動で境界値に補正'],
                ['補正値合計', 'サーバー', '-3〜+3の範囲（clamp）', '範囲外は自動で境界値に補正'],
                ['チーム数', 'サーバー', '2以上（max(2, 入力値)）', '1以下は2として扱う'],
                ['チーム分け実行時のプレイヤー数', 'サーバー', '1人以上いること', '0人ならエラーレスポンス'],
              ]" :key="v[0]" class="hover:bg-neutral-50">
                <td class="border border-neutral-200 px-3 py-2 font-medium">{{ v[0] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ v[1] }}</td>
                <td class="border border-neutral-200 px-3 py-2">{{ v[2] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ v[3] }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

    </div>
  </div>
</template>
