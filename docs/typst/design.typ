#import "@preview/chronos:0.2.1"
#set text(font: "Noto Serif JP", lang: "ja")

= 設計内容の概要

- 現在開いているページの内容を辞書検索し，特定のキーワードを抽出する。
  - DOM操作でサイトのテキスト部分を取得し，辞書検索からフィルタリングする。
  - 辞書内容はサーバーに保存しておき，APIで取得する。
- サーバーはキーワードを受け取り，サイトの種類によって計算方式を変えながら雰囲気指数を計算する。
  - YouTubeはタグまたはタイトルをキーワードとする。
  - Netflixタイトルをキーワードとする。タイトルをネットで検索するなどの処理を行い，雰囲気のわかるタグを取得する
  - その他サイトは，辞書検索でヒットしたワードをキーワードとする。
- 雰囲気指数を照度に変換する。
- Nature Remo APIを用いて照度を家電に適応する。
- パソコンの電源を落とした場合、現在の明るさを保持する。

= システム処理の流れ

#v(10pt)システム処理の流れを簡易的にモデル化したものを下に示す。

== シーケンス図#v(10pt)
#chronos.diagram({
  import chronos: *
  _seq("User", "Extension", comment: "ページを開く")
  _seq("Extension", "Extension", comment: "WebページのDOMを取得")
  _seq("AtmosServer", "Extension", comment: "辞書を取得")

  _seq(
    "Extension",
    "Extension",
    comment: "辞書検索でDOM内容から\nキーワードを抽出",
  )
  _seq("Extension", "AtmosServer", comment: "キーワードを送信")

  _seq(
    "AtmosServer",
    "AtmosServer",
    comment: "受け取ったデータから\n雰囲気指数を算出",
  )
  _seq("AtmosServer", "NatureRemo", comment: "照度取得をリクエスト")
  _seq("NatureRemo", "AtmosServer", comment: "照度をレスポンス")
  _seq("AtmosServer", "AtmosServer", comment: "目標照度を計算")
  _seq("AtmosServer", "NatureRemo", comment: "照度を家電に適応")
})

#pagebreak()

== データフロー図

#figure(image("dataflow.png", width: 120%))

= 必要なモジュール

Atmosサーバーはヘキサゴナルアーキテクチャを採用している。

- atmos_server
  - domain.rs
    - models.rs
      - atmosfreq.rs
      - lighting.rs
    - ports.rs:
      - lighting.rs
    - service.rs
      - lighting.rs
  - inbound.rs: APIの定義
    - http.rs
      - api.rs
      - routes.rs
        - adjust_lighting.rs
        - get_lighting_signals.rs
  - outbount.rs: 外部とのやりとり
    - remo.rs

- atmos_extension
  - popup/
    - App.svelte: 設定の表示
  - content.ts: DOMの操作
