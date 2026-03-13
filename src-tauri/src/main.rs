// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod utils;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 注册全局快捷键
            commands::hotkey::register_global_shortcuts(&app.handle())
                .map_err(|e| {
                    eprintln!("Failed to register global shortcuts: {}", e);
                    e
                })
                .ok();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::disk::get_disk_drives,
            commands::disk::get_files_in_directory,
            commands::disk::open_file_with_default_app,
            commands::window::toggle_window,
            commands::window::set_window_height,
            commands::window::expand_window,
            commands::window::collapse_window,
            commands::window::set_always_on_top,
            commands::window::get_window_position,
            commands::window::minimize_to_fab,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}