use winapi::um::fileapi::{GetLogicalDriveStringsW, GetDriveTypeW};
use winapi::um::winbase::DRIVE_FIXED;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::process::Command;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

/// 使用系统默认程序打开文件
#[tauri::command]
pub async fn open_file_with_default_app(path: String) -> Result<(), String> {
    let path_obj = Path::new(&path);

    if !path_obj.exists() {
        return Err(format!("File does not exist: {}", path));
    }

    // Windows 上使用 start 命令打开文件
    Command::new("cmd")
        .args(&["/C", "start", "", &path])
        .spawn()
        .map_err(|e| format!("Failed to open file: {}", e))?;

    Ok(())
}

/// 获取所有固定驱动器的盘符
#[tauri::command]
pub async fn get_disk_drives() -> Result<Vec<String>, String> {
    let mut buffer = [0u16; 256];
    unsafe {
        let result = GetLogicalDriveStringsW(buffer.len() as u32, buffer.as_mut_ptr());

        if result == 0 {
            return Err("Failed to get logical drive strings".to_string());
        }

        let drives: Vec<String> = buffer
            .split(|&c| c == 0)
            .filter(|s| !s.is_empty())
            .map(|s| {
                OsString::from_wide(s)
                    .to_string_lossy()
                    .trim_end_matches('\\')
                    .to_uppercase()
            })
            .filter(|drive| {
                // 只返回固定驱动器
                let mut drive_string: Vec<u16> = drive.encode_utf16().collect();
                drive_string.push(0); // null terminator
                drive_string.extend_from_slice(&[b'\\' as u16, 0]); // Add backslash and null terminator

                let drive_type = GetDriveTypeW(drive_string.as_ptr());
                drive_type == DRIVE_FIXED
            })
            .collect();

        Ok(drives)
    }
}

/// 获取指定目录下的文件列表
#[tauri::command]
pub async fn get_files_in_directory(path: String) -> Result<Vec<FileInfo>, String> {
    let path_obj = match Path::new(&path) {
        p if p.exists() => p,
        _ => return Err(format!("Path does not exist: {}", path))
    };

    if !path_obj.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }

    let mut files = Vec::new();

    // 使用 standard library 读取目录
    use std::fs;

    let entries = fs::read_dir(&path_obj)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let metadata = entry.metadata().map_err(|e| format!("Failed to get metadata: {}", e))?;

        let file_path = entry.path();
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let is_dir = metadata.is_dir();
        let size = if is_dir { 0 } else { metadata.len() };

        files.push(FileInfo {
            path: file_path.to_string_lossy().to_string(),
            name: file_name,
            size,
            is_dir,
        });
    }

    // 先显示文件夹，再显示文件
    files.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });

    Ok(files)
}