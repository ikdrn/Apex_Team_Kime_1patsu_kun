<script setup lang="ts">
import { useRouter } from 'vue-router'
const router = useRouter()
</script>

<template>
  <div class="min-h-screen bg-neutral-100">
    <header class="bg-blue-700 text-white px-6 py-4 flex items-center gap-4 shadow">
      <button @click="router.push('/owner/docs')" class="text-blue-200 hover:text-white transition-colors text-sm">← 設計書一覧に戻る</button>
      <div class="w-px h-4 bg-blue-500" />
      <h1 class="text-base font-bold">🏗️ 基本設計書</h1>
      <div class="flex-1" />
      <span class="text-xs text-blue-200">バージョン 1.0 ／ 2025年作成</span>
    </header>

    <div class="max-w-4xl mx-auto px-6 py-8 space-y-8">

      <!-- 1. システム構成 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">1. システム全体の構成図</h2>
        </div>
        <div class="p-5">
          <p class="text-xs text-neutral-500 mb-5">このシステムは「フロントエンド（見た目）」と「バックエンド（処理）」の2層構造になっています。</p>

          <!-- 構成図 -->
          <div class="border border-neutral-200 p-4 bg-neutral-50">
            <div class="flex flex-col items-center gap-2">

              <!-- ブラウザ層 -->
              <div class="w-full max-w-2xl">
                <div class="text-xs font-bold text-neutral-500 mb-2 text-center tracking-wider">【 ブラウザ（利用者の画面） 】</div>
                <div class="grid grid-cols-2 gap-3">
                  <div class="border-2 border-blue-400 bg-blue-50 p-3 text-center">
                    <p class="text-xs font-bold text-blue-700">一般ユーザー画面</p>
                    <p class="text-[10px] text-blue-600 mt-1">URL: /</p>
                    <p class="text-[10px] text-neutral-500 mt-1">参加登録・チーム結果閲覧</p>
                  </div>
                  <div class="border-2 border-neutral-800 bg-neutral-50 p-3 text-center">
                    <p class="text-xs font-bold text-neutral-800">管理者画面</p>
                    <p class="text-[10px] text-neutral-600 mt-1">URL: /owner</p>
                    <p class="text-[10px] text-neutral-500 mt-1">参加者管理・チーム分け実行</p>
                  </div>
                </div>
                <div class="mt-2 text-center text-xs text-neutral-500 bg-blue-100 border border-blue-200 p-2">
                  Vue.js（JavaScript フレームワーク）で作られた画面。Tailwind CSS でデザイン。
                </div>
              </div>

              <!-- 矢印 -->
              <div class="flex flex-col items-center text-neutral-400">
                <div class="w-px h-4 bg-neutral-300" />
                <div class="text-xs font-bold border border-neutral-300 bg-white px-3 py-1">HTTP通信（API）</div>
                <div class="text-[10px] text-neutral-400">リクエスト↓ / レスポンス↑</div>
                <div class="w-px h-4 bg-neutral-300" />
              </div>

              <!-- バックエンド層 -->
              <div class="w-full max-w-2xl">
                <div class="text-xs font-bold text-neutral-500 mb-2 text-center tracking-wider">【 サーバー（処理を担当） 】</div>
                <div class="border-2 border-orange-400 bg-orange-50 p-4">
                  <p class="text-xs font-bold text-orange-700 text-center mb-3">Rust + Axum バックエンドサーバー</p>
                  <div class="grid grid-cols-3 gap-2">
                    <div class="bg-white border border-orange-200 p-2 text-center text-[10px]">
                      <p class="font-bold text-neutral-700">プレイヤー管理</p>
                      <p class="text-neutral-500 mt-0.5">登録・更新・削除</p>
                    </div>
                    <div class="bg-white border border-orange-200 p-2 text-center text-[10px]">
                      <p class="font-bold text-neutral-700">チーム分けアルゴリズム</p>
                      <p class="text-neutral-500 mt-0.5">30回試行・最良選択</p>
                    </div>
                    <div class="bg-white border border-orange-200 p-2 text-center text-[10px]">
                      <p class="font-bold text-neutral-700">データ保持（メモリ）</p>
                      <p class="text-neutral-500 mt-0.5">プレイヤー・チーム・設定</p>
                    </div>
                  </div>
                  <p class="text-[10px] text-orange-600 mt-2 text-center">PORT環境変数で待ち受けポートを設定（デフォルト8080）</p>
                </div>
              </div>

              <!-- デプロイ情報 -->
              <div class="w-full max-w-2xl mt-1">
                <div class="text-xs font-bold text-neutral-500 mb-2 text-center tracking-wider">【 クラウド環境 】</div>
                <div class="border border-violet-300 bg-violet-50 p-3 text-center">
                  <p class="text-xs font-bold text-violet-700">Render.com（クラウドサービス）</p>
                  <p class="text-[10px] text-violet-600 mt-1">フロントエンドとバックエンドを1つのDockerコンテナとして動作。自動デプロイ対応。</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- 2. 画面遷移図 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">2. 画面遷移図（どの画面からどこへ移動できるか）</h2>
        </div>
        <div class="p-5">
          <div class="border border-neutral-200 p-4 bg-neutral-50">
            <div class="flex flex-col items-center gap-3">

              <!-- 入口 -->
              <div class="border-2 border-neutral-400 bg-white px-6 py-2 text-xs font-bold text-neutral-700">
                🌐 URL でアクセス
              </div>

              <div class="flex gap-16 items-start">
                <!-- ユーザー側 -->
                <div class="flex flex-col items-center gap-2">
                  <div class="text-[10px] text-neutral-400">URL: /</div>
                  <div class="border-2 border-blue-400 bg-blue-50 p-3 text-center w-36">
                    <p class="text-xs font-bold text-blue-700">一般ユーザー画面</p>
                    <p class="text-[10px] text-neutral-500 mt-1">参加登録・チーム閲覧</p>
                  </div>
                  <div class="w-px h-3 bg-neutral-300" />
                  <div class="border border-blue-200 bg-white p-2 text-center text-[10px] w-36">
                    <p>登録後：自分の情報を編集可能</p>
                  </div>
                </div>

                <!-- 管理者側 -->
                <div class="flex flex-col items-center gap-2">
                  <div class="text-[10px] text-neutral-400">URL: /owner</div>
                  <div class="border-2 border-neutral-800 bg-neutral-50 p-3 text-center w-36">
                    <p class="text-xs font-bold text-neutral-800">管理者画面</p>
                    <p class="text-[10px] text-neutral-500 mt-1">全機能を操作可能</p>
                  </div>
                  <div class="w-px h-3 bg-neutral-300" />
                  <div class="border border-neutral-300 bg-white p-2 text-center text-[10px] w-36">
                    <p>ヘッダー「設計書」ボタン</p>
                  </div>
                  <div class="w-px h-3 bg-neutral-300" />
                  <div class="border border-violet-300 bg-violet-50 p-2 text-center text-[10px] w-36">
                    <p class="font-bold text-violet-700">URL: /owner/docs</p>
                    <p>設計書一覧</p>
                  </div>
                  <div class="w-px h-3 bg-neutral-300" />
                  <div class="border border-violet-200 bg-white p-2 text-center text-[10px] w-36">
                    <p>各設計書ページ（9種類）</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- 3. API一覧 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">3. APIエンドポイント一覧（フロントエンドとバックエンドの通信窓口）</h2>
        </div>
        <div class="p-5">
          <p class="text-xs text-neutral-500 mb-4">「API」とは、フロントエンド（画面）とバックエンド（サーバー）がデータをやりとりするための窓口です。</p>
          <table class="w-full text-xs border-collapse">
            <thead>
              <tr class="bg-neutral-100 text-neutral-600">
                <th class="border border-neutral-200 px-3 py-2 text-left">メソッド</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">URL</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">何をするAPI？</th>
                <th class="border border-neutral-200 px-3 py-2 text-center">誰が使う？</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="api in [
                ['GET',    '/api/players',              'プレイヤー一覧を取得する',              '全員'],
                ['POST',   '/api/players',              '新しいプレイヤーを追加する',            '全員'],
                ['PUT',    '/api/players/:id',          '指定プレイヤーの名前・ランクを変更する', '全員*'],
                ['DELETE', '/api/players/:id',          '指定プレイヤーを削除する',              '管理者'],
                ['PATCH',  '/api/players/:id/offset',   '戦闘力補正値を±1調整する',             '管理者'],
                ['GET',    '/api/config',               '現在のチーム数設定を取得する',           '全員'],
                ['PUT',    '/api/config',               'チーム数を変更する',                   '管理者'],
                ['POST',   '/api/teams/balance',        'チーム分けを実行する',                  '管理者'],
                ['GET',    '/api/teams',                'チーム分け結果を取得する',              '全員'],
                ['DELETE', '/api/teams',                'チーム分け結果をリセットする',           '管理者'],
                ['GET',    '/health',                   'サーバーの死活監視',                   'システム'],
              ]" :key="api[1]" class="hover:bg-neutral-50">
                <td class="border border-neutral-200 px-3 py-2">
                  <span class="font-mono font-bold text-[10px] px-1.5 py-0.5"
                    :class="{
                      'bg-emerald-100 text-emerald-700': api[0]==='GET',
                      'bg-blue-100 text-blue-700': api[0]==='POST',
                      'bg-amber-100 text-amber-700': api[0]==='PUT' || api[0]==='PATCH',
                      'bg-red-100 text-red-700': api[0]==='DELETE',
                    }">{{ api[0] }}</span>
                </td>
                <td class="border border-neutral-200 px-3 py-2 font-mono text-[10px] text-neutral-600">{{ api[1] }}</td>
                <td class="border border-neutral-200 px-3 py-2">{{ api[2] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-center">{{ api[3] }}</td>
              </tr>
            </tbody>
          </table>
          <p class="text-xs text-neutral-400 mt-2">* 一般ユーザーは自分自身のIDを知っている場合のみ変更可能（フロントエンドで制御）</p>
        </div>
      </section>

      <!-- 4. データ設計 -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">4. データ設計（扱うデータの種類と関係）</h2>
        </div>
        <div class="p-5 space-y-4">
          <p class="text-xs text-neutral-500">サーバーのメモリ上で管理するデータ構造です。サーバー再起動でリセットされます。</p>

          <!-- データ構造図 -->
          <div class="border border-neutral-200 p-4 bg-neutral-50 space-y-3">

            <!-- AppState -->
            <div class="border-2 border-neutral-700 bg-white p-4">
              <p class="text-xs font-bold text-neutral-700 mb-3">📦 アプリ全体の状態（AppState）</p>
              <div class="grid grid-cols-3 gap-3">
                <!-- players -->
                <div class="border border-blue-300 bg-blue-50 p-3">
                  <p class="text-[10px] font-bold text-blue-700 mb-2">📋 プレイヤーリスト</p>
                  <div class="space-y-1 text-[10px]">
                    <div class="flex gap-2"><span class="text-neutral-400 font-mono">id</span><span>UUID（自動生成）</span></div>
                    <div class="flex gap-2"><span class="text-neutral-400 font-mono">name</span><span>プレイヤー名（30文字以内）</span></div>
                    <div class="flex gap-2"><span class="text-neutral-400 font-mono">rank</span><span>ランク（7種類）</span></div>
                    <div class="flex gap-2"><span class="text-neutral-400 font-mono">score_offset</span><span>補正値（−3〜+3）</span></div>
                    <div class="flex gap-2"><span class="text-neutral-400 font-mono">display_name</span><span>重複解決後の名前</span></div>
                  </div>
                </div>
                <!-- teams -->
                <div class="border border-orange-300 bg-orange-50 p-3">
                  <p class="text-[10px] font-bold text-orange-700 mb-2">🏆 チームリスト（任意）</p>
                  <div class="space-y-1 text-[10px]">
                    <div class="flex gap-2"><span class="text-neutral-400 font-mono">id</span><span>チーム番号（1〜）</span></div>
                    <div class="flex gap-2"><span class="text-neutral-400 font-mono">players</span><span>所属プレイヤー一覧</span></div>
                    <div class="flex gap-2"><span class="text-neutral-400 font-mono">total_score</span><span>チーム合計スコア</span></div>
                  </div>
                  <p class="text-[10px] text-orange-600 mt-2">※ チーム分け未実行時は null</p>
                </div>
                <!-- config -->
                <div class="border border-emerald-300 bg-emerald-50 p-3">
                  <p class="text-[10px] font-bold text-emerald-700 mb-2">⚙️ 設定</p>
                  <div class="space-y-1 text-[10px]">
                    <div class="flex gap-2"><span class="text-neutral-400 font-mono">team_count</span><span>チーム数（2〜6、初期値:2）</span></div>
                  </div>
                </div>
              </div>
            </div>

            <!-- ランクとスコアの対応表 -->
            <div class="border border-neutral-200 bg-white p-4">
              <p class="text-[10px] font-bold text-neutral-700 mb-2">📊 ランクと基本スコアの対応</p>
              <div class="flex gap-2 flex-wrap">
                <div v-for="r in [
                  {name:'ブロンズ', score:10, bg:'bg-amber-100', text:'text-amber-800'},
                  {name:'シルバー', score:20, bg:'bg-slate-100', text:'text-slate-700'},
                  {name:'ゴールド', score:30, bg:'bg-yellow-100', text:'text-yellow-800'},
                  {name:'プラチナ', score:40, bg:'bg-cyan-100', text:'text-cyan-800'},
                  {name:'ダイヤモンド', score:50, bg:'bg-blue-100', text:'text-blue-800'},
                  {name:'マスター', score:60, bg:'bg-purple-100', text:'text-purple-800'},
                  {name:'プレデター', score:70, bg:'bg-red-100', text:'text-red-800'},
                ]" :key="r.name" class="text-center">
                  <div :class="[r.bg, r.text, 'text-[10px] font-bold px-2 py-1 border border-neutral-200']">{{ r.name }}</div>
                  <div class="text-[11px] font-black text-neutral-800 mt-0.5">{{ r.score }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- 5. エラーハンドリング -->
      <section class="bg-white border border-neutral-200 shadow-sm">
        <div class="bg-blue-600 text-white px-5 py-3">
          <h2 class="font-bold text-sm tracking-wider">5. エラーハンドリング方針（エラーが起きたときの対応）</h2>
        </div>
        <div class="p-5">
          <table class="w-full text-xs border-collapse">
            <thead>
              <tr class="bg-neutral-100 text-neutral-600">
                <th class="border border-neutral-200 px-3 py-2 text-left">状況</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">HTTPステータス</th>
                <th class="border border-neutral-200 px-3 py-2 text-left">ユーザーへの表示</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="e in [
                ['プレイヤー名が空', '400 Bad Request', '「プレイヤー名を入力してください」と画面下部にトースト表示（5秒後自動消去）'],
                ['プレイヤー名が30文字超', '400 Bad Request', '「プレイヤー名は30文字以内で入力してください」とトースト表示'],
                ['プレイヤーが0人でチーム分け実行', '400 Bad Request', '「プレイヤーが登録されていません」とトースト表示'],
                ['存在しないIDへの操作', '404 Not Found', 'エラーメッセージをトースト表示'],
                ['チーム分け未実行でGET /api/teams', '404 Not Found', 'フロントエンドではエラーとして扱わず、チーム未表示状態を維持'],
                ['ネットワークエラー', 'なし（通信失敗）', '操作に応じたエラーメッセージをトースト表示'],
              ]" :key="e[0]" class="hover:bg-neutral-50">
                <td class="border border-neutral-200 px-3 py-2 font-medium">{{ e[0] }}</td>
                <td class="border border-neutral-200 px-3 py-2 font-mono text-neutral-600">{{ e[1] }}</td>
                <td class="border border-neutral-200 px-3 py-2 text-neutral-600">{{ e[2] }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

    </div>
  </div>
</template>
