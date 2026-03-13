use tauri::{AppHandle, GlobalShortcutManager};

/// 注册全局快捷键
pub fn register_global_shortcuts(app: &AppHandle) -> Result<(), String> {
    let mut shortcut_manager = app.global_shortcut_manager();

    // 注册 Ctrl+Alt+D 作为切换快捷键
    let toggle_shortcut = "Ctrl+Alt+D";
    shortcut_manager
        .register(toggle_shortcut, move || {
            // 触发快捷键事件
            println!("Toggle shortcut pressed");
        })
        .map_err(|e| format!("Failed to register toggle shortcut: {}", e))?;

    // 注册 Alt + ` 作为快速显示快捷键
    let show_shortcut = "Alt+`";
    shortcut_manager
        .register(show_shortcut, move || {
            // 触发显示事件
            println!("Show shortcut pressed");
        })
        .map_err(|e| format!("Failed to register show shortcut: {}", e))?;

    Ok(())
}

/// 解除快捷键注册
pub fn unregister_global_shortcuts(app: &AppHandle) -> Result<(), String> {
    let mut shortcut_manager = app.global_shortcut_manager();

    let toggle_shortcut = "Ctrl+Alt+D";
    let show_shortcut = "Alt+`";

    shortcut_manager
        .unregister(toggle_shortcut)
        .map_err(|e| format!("Failed to unregister toggle shortcut: {}", e))?;

    shortcut_manager
        .unregister(show_shortcut)
        .map_err(|e| format!("Failed to unregister show shortcut: {}", e))?;

    Ok(())
}