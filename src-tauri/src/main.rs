#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;
mod system;
mod types;
mod window;

use state::AppState;
use tauri::Manager;
use window::setup_window_effects;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            setup_window_effects(&window).expect("Failed to apply window effects");
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::get_processes,
            commands::kill_process
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
