# keyboardrum

Your keyboard zones become a drum kit. As you type — in prose, in code — each key
fires a drum hit, so ordinary typing turns into a beat. Space is the kick (foot),
the left home row is the snare, the right home row is the hi-hat, and so on.

> **keyboard** + **drum**, welded at the shared `d`.

日本語: タイプするたびにドラムが鳴る常駐楽器。キーボードの「手の領域」がドラムキットに
割り当てられていて、普通に文章やコードを打つだけで「どんどんどん」とビートになる。

## いま試す（ビルド不要・ブラウザ）

```sh
# どれでもOK
xdg-open frontend/index.html         # ファイルを直接開く
# または静的サーバ経由
npm run dev                          # http://localhost:1420
```

ウィンドウにフォーカスした状態でタイプすると鳴ります（試作はフォーカス専用）。

## ネイティブ（Tauri）

```sh
npm install
npm run tauri:dev
```

## 領域マッピング（試作の既定）

| ゾーン | パーツ | キー |
|---|---|---|
| スペース（親指/足） | キック | `Space` |
| 左ホーム | スネア | `A S D F G` |
| 左上 | ハイタム | `Q W E R T` |
| 左下 | フロアタム | `Z X C V B` |
| 右ホーム | ハイハット | `H J K L ;` |
| 右上 | ライド | `Y U I O P` |
| 右下 | リム/クリック | `N M , . /` `Backspace` |
| 記号/数字 | オープンハット | `Tab [ ] \ '` |
| Enter・数字行 | クラッシュ | `Enter` `1`–`0` ほか |

## 発音ロジック

- **異なるパーツの同時押し → 両方鳴る**（ポリフォニック）
- **同一パーツに当たるキーを2つ同時 → 1回だけ**（同一パーツの合流, ~14ms 窓）
- **長押しのオートリピートは無視**（1押下＝1打）

## いまの状態

- 音は**手続き合成（Web Audio）の仮ドラム**。ライセンス無縁で即発音するため。
  本命は**生ドラム（AVL Drumkits 等のアコースティック・サンプル）＋ラウンドロビン**に差し替え。
- 試作は**フォーカス専用**。本命の「常駐して全アプリで鳴る」は Rust 側の
  グローバルキーフック（Wayland 制約 → X11/evdev 等）を後段で足す。

詳細仕様は [DESIGN.md](./DESIGN.md)。
