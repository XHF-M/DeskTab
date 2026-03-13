use tauri::Window;

/// 切换窗口显示/隐藏
#[tauri::command]
pub async fn toggle_window(window: Window) -> Result<(), String> {
    if window.is_visible().map_err(|e| e.to_string())? {
        window.hide().map_err(|e| e.to_string())?;
    } else {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 设置窗口高度（用于展开/收缩）
#[tauri::command]
pub async fn set_window_height(window: Window, height: u32) -> Result<(), String> {
    window.set_size(tauri::Size::Logical(tauri::LogicalSize {
        width: 800.0,
        height: height as f64,
    }))
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// 展开窗口
#[tauri::command]
pub async fn expand_window(window: Window, height: Option<u32>) -> Result<(), String> {
    let target_height = height.unwrap_or(350) as f64;
    window.set_size(tauri::Size::Logical(tauri::LogicalSize {
        width: 800.0,
        height: 400.0,
    }))
    .map_err(|e| e.to_string())?;

    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;

    Ok(())
}

/// 收缩窗口
#[tauri::command]
pub async fn collapse_window(window: Window) -> Result<(), String> {
    window.set_size(tauri::Size::Logical(tauri::LogicalSize {
        width: 800.0,
        height: 80.0,
    }))
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// 设置窗口始终置顶
#[tauri::command]
pub async fn set_always_on_top(window: Window, always_on_top: bool) -> Result<(), String> {
    window.set_always_on_top(always_on_top).map_err(|e| e.to_string())
}

/// 获取窗口当前位置
#[tauri::command]
pub async fn get_window_position(window: Window) -> Result<(i32, i32), String> {
    let position = window.outer_position().map_err(|e| e.to_string())?;
    Ok((position.x, position.y))
}

/// 收缩为悬浮按钮
#[tauri::command]
pub async fn minimize_to_fab(window: Window) -> Result<(), String> {
    window
        .set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: 60.0,
            height: 60.0,
        }))
        .map_err(|e| e.to_string())?;

    Ok(())
}