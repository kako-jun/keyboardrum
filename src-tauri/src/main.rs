#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 試作: 通常ウィンドウ内の webview がフォーカス中の打鍵を拾って鳴らす（ステージ1 = フォーカス専用）。
// 「常駐して全アプリで鳴る」本命は、後で Rust 側のグローバルキーフック（rdev 等）を足す。
// ただし Wayland は全キーの盗聴を原則禁止 → X11 / evdev 直読み or プラットフォーム別実装が必要。
fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running keyboardrum");
}
