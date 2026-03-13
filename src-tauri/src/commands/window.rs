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
    let target_height = 400.0;
    let target_width = 800.0;

    // 获取当前窗口位置
    let current_position = window.outer_position().map_err(|e| e.to_string())?;
    let current_x = current_position.x;
    let current_y = current_position.y;

    // 获取屏幕尺寸
    let monitor = window.current_monitor().map_err(|e| e.to_string())?.ok_or("No monitor found")?;
    let screen_size = monitor.size();
    let screen_width = screen_size.width as i32;
    let screen_height = screen_size.height as i32;

    // 计算展开后的窗口位置，确保不超出屏幕
    let mut new_x = current_x;
    let mut new_y = current_y;

    // 右边界检查
    if new_x + target_width as i32 > screen_width {
        new_x = (screen_width - target_width as i32).max(0);
    }

    // 下边界检查
    if new_y + target_height as i32 > screen_height {
        new_y = (screen_height - target_height as i32).max(0);
    }

    // 左边界检查
    if new_x < 0 {
        new_x = 0;
    }

    // 上边界检查
    if new_y < 0 {
        new_y = 0;
    }

    // 设置窗口位置和大小
    window
        .set_position(tauri::Position::Logical(tauri::LogicalPosition {
            x: new_x as f64,
            y: new_y as f64,
        }))
        .map_err(|e| e.to_string())?;

    window
        .set_size(tauri::Size::Logical(tauri::LogicalSize {
            width: target_width,
            height: target_height,
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