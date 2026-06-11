# keyboardrum

Your keyboard zones become a drum kit. As you type — in prose, in code — each key
fires a drum hit, so ordinary typing turns into a beat. Space is the kick (foot),
the left home row is the snare, the right home row is the hi-hat, and so on.

> **keyboard** + **drum**, welded at the shared `d`.

日本語: タイプするたびにドラムが鳴る常駐楽器。キーボードの「手の領域」がドラムキットに
割り当てられていて、普通に文章やコードを打つだけで「どんどんどん」とビートになる。

## いま試す（ビルド不要・ブラウザ）

```sh
npm run dev   # 静的配信。表示される Local / Network の URL をブラウザで開く
```

- ローカルなら `http://localhost:1420`。
- **リモート開発（SSH 等）**なら、表示される **Network** の URL（`http://<dev-host>:1420`）を手元のブラウザで開く。
  Web Audio もキー入力も手元側で動くので、音は手元のスピーカーから鳴る。
- 単に `frontend/index.html` をブラウザで直接開いてもよい（ローカルのみ）。

ブラウザのウィンドウにフォーカスした状態でタイプすると鳴ります（試作はフォーカス専用）。

## ネイティブ（Tauri）

```sh
npm install
npm run tauri:dev
```

## 領域マッピング（実機ドラムキット準拠）

記号はドラム譜の慣習（● 打面 / × シンバル・ハット / ○ オープンハット / ⊗ クロススティック）。
実機どおり **ハイハット=左・ライド=右**。**どの物理キーも鳴る**（未マップは ⊗ にフォールバック）。

| 実機の位置 | パーツ | 記号 | キー |
|---|---|---|---|
| 足 | キック | ● | `Space` |
| 左ホーム | スネア | ● | `A S D F G` |
| 左上 | ハイハット（左手） | × | `Q W E R T` |
| 左端・足 | ハイハット（ペダル） | × | `Tab Caps LShift LCtrl LAlt LWin` |
| 左下 | ハイタム | ● | `Z X C V B` `↑` |
| 右ホーム | ライド（右手） | × | `H J K L ;` |
| 右下 | フロアタム | ● | `N M , . /` `↓` |
| 右端 | オープンハット | ○ | `[ ] \ '` |
| 右上・数字・Esc・F行・端 | クラッシュ | × | `Y U I O P` `Enter` `1`–`0` `Esc` `F1`–`F12` `RShift/RCtrl/RAlt` |
| ナビ | クロススティック | ⊗ | `Delete Home End PgUp PgDn ← →` |

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
