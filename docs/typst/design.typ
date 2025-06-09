#import "@preview/chronos:0.2.1"
#set text(font: "Noto Serif CJK JP", lang: "ja")

= 設計内容の概要


- URLを用いてパソコンの画面の情報をパソコンの画面が変わるごとに取得し、雰囲気指数をバックエンド内で算出する。
- Remo 3を用いて電気の照度をパソコンの画面が変わるごとに取得し、バックエンドに送信する。
- Remo 3を用いて現在の電気の照度を取得し、バックエンドに送信する。
- 雰囲気指数が変化した場合、以下のようにRemo 3、電気を操作する。
  - 電気の電源が入っておらず、かつ雰囲気指数が変わった場合は、変化は起きないため電気の電源を入れる必要がある。
  - パソコンの画面が変更された場合は、雰囲気指数から目標の明るさをバックエンドで設定する。Remo 3から取得される現在の明るさを目標の明るさにRemo 3を通じて調節する。
- パソコンの電源を落とした場合、現在の明るさを保持する。



小説サイトを開いた場合
= システム処理の流れ

#v(10pt)システム処理の流れを簡易的にモデル化したものを下に示す。\
\
== シーケンス図#v(10pt)
#chronos.diagram({
  import chronos: *
  _seq("User", "Extension", comment: "ページを開く")
  _seq("Extension", "Extension", comment: "WebページのHTMLを取得")
  _seq(
    "Extension",
    "Extension",
    comment: "HTMLからサーバーに\n送信するフォーマットにパース",
  )
  _seq("Extension", "Server", comment: "パースしたデータをサーバーに送信")

  _seq("Server", "Server", comment: "受け取ったデータから\n雰囲気指数を算出")
  _seq("Server", "NatureRemo", comment: "明るさ指数取得をリクエスト")
  _seq("NatureRemo", "Server", comment: "明るさ指数をレスポンス")
  _seq(
    "Server",
    "Server",
    comment: "明るさ指数と雰囲気指数から、\n明るさを上げるか下げるか決定",
  )
})



#chronos.diagram({
  import chronos: *

  _seq("Server", "Server", comment: "受け取ったデータから雰囲気指数を算出")
  _seq("Server", "NatureRemo", comment: "明るさ指数取得をリクエスト")
  _seq("NatureRemo", "Server", comment: "明るさ指数をレスポンス")
  _seq(
    "Server",
    "Server",
    comment: "明るさ指数と雰囲気指数から、明るさを上げるか下げるか決定",
  )
})
#pagebreak()
== データフロー図
= 必要なモジュール（.gs ファイル）
