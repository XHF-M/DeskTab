# DeskTab

一个基于 Tauri + Vue 3 构建的轻量级桌面文件管理工具，悬浮于桌面之上，随时快速访问本地文件。

## 功能特性

- **悬浮窗口** - 无边框、透明背景、始终置顶，不影响正常工作流程
- **智能收缩** - 可收缩为 60x60 的圆形悬浮按钮，节省屏幕空间
- **拖拽移动** - 支持窗口拖拽，自由放置在屏幕任意位置
- **磁盘浏览** - 自动检测本地固定磁盘驱动器，一键切换浏览
- **文件管理** - 浏览目录结构，双击文件用系统默认程序打开
- **搜索过滤** - 支持当前目录内文件名搜索
- **全局快捷键** - 支持 `Ctrl+Alt+D` 和 `Alt+`` 快捷键操作

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | Vue 3 + TypeScript |
| 构建工具 | Vite 5 |
| 状态管理 | Pinia |
| 组合式函数 | @vueuse/core |
| 桌面框架 | Tauri 1.5 |
| 后端语言 | Rust |
| 系统接口 | WinAPI |

## 环境要求

- **Node.js** >= 18.0
- **Rust** >= 1.70
- **Windows** 10/11（仅支持 Windows 平台）

## 安装和运行

### 1. 克隆项目

```bash
git clone
cd desktab
```

### 2. 安装依赖

```bash
npm install
```

### 3. 开发模式运行

```bash
npm run tauri:dev
```

该命令会同时启动 Vite 开发服务器和 Tauri 应用窗口。

### 4. 构建生产版本

```bash
npm run tauri:build
```

构建完成后，安装包位于 `src-tauri/target/release/bundle/` 目录下。

## 项目结构

```
desktab/
├── src/                        # Vue 前端源码
│   ├── components/             # Vue 组件
│   │   ├── DiskTab.vue         # 磁盘标签组件
│   │   ├── FileList.vue        # 文件列表组件
│   │   ├── TopBar.vue          # 顶部栏/悬浮按钮组件
│   │   └── HotkeyHandler.vue   # 快捷键处理组件
│   ├── stores/                 # Pinia 状态管理
│   │   ├── diskStore.ts        # 磁盘状态
│   │   └── windowStore.ts      # 窗口状态
│   ├── hooks/                  # 组合式函数
│   │   ├── useDiskDrives.ts    # 磁盘驱动器 Hook
│   │   └── useWindowControl.ts # 窗口控制 Hook
│   ├── types/                  # TypeScript 类型定义
│   ├── styles/                 # 样式文件
│   │   ├── variables.css       # CSS 变量主题
│   │   └── global.css          # 全局样式
│   ├── App.vue                 # 根组件
│   └── main.ts                 # 入口文件
├── src-tauri/                  # Rust 后端源码
│   ├── src/
│   │   ├── commands/           # Tauri 命令
│   │   │   ├── disk.rs         # 磁盘操作命令
│   │   │   ├── window.rs       # 窗口控制命令
│   │   │   └── hotkey.rs       # 快捷键命令
│   │   ├── lib.rs              # 库入口
│   │   └── main.rs             # 应用入口
│   ├── icons/                  # 应用图标
│   ├── Cargo.toml              # Rust 依赖配置
│   └── tauri.conf.json         # Tauri 配置
├── package.json
├── vite.config.ts
└── tsconfig.json
```

## 核心功能说明

### 窗口模式

DeskTab 有两种显示模式：

| 模式 | 尺寸 | 说明 |
|------|------|------|
| 悬浮按钮 | 60x60 | 最小化为圆形悬浮按钮，点击可展开 |
| 完整窗口 | 800x350 | 显示磁盘标签和文件列表 |

### 磁盘驱动器检测

后端使用 WinAPI 获取本地固定磁盘驱动器：

```rust
// 仅返回 DRIVE_FIXED 类型的驱动器
let drive_type = GetDriveTypeW(drive_string.as_ptr());
drive_type == DRIVE_FIXED
```

### 文件操作

| 操作 | 说明 |
|------|------|
| 单击 | 选中文件 |
| 双击文件夹 | 进入目录 |
| 双击文件 | 用系统默认程序打开 |

### 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+Alt+D` | 切换窗口显示/隐藏 |
| `Alt+`` | 快速显示窗口 |

## 配置说明

### Tauri 窗口配置

```json
// src-tauri/tauri.conf.json
{
  "windows": [{
    "title": "DeskTab",
    "width": 800,
    "height": 400,
    "resizable": false,
    "decorations": false,  // 无边框
    "transparent": true,   // 透明背景
    "alwaysOnTop": true,   // 始终置顶
    "skipTaskbar": true    // 不显示在任务栏
  }]
}
```

### CSS 主题变量

```css
/* src/styles/variables.css */
:root {
  --background-primary: rgba(30, 30, 30, 0.95);
  --background-hover: rgba(50, 50, 50, 0.95);
  --text-primary: #ffffff;
  --text-secondary: rgba(255, 255, 255, 0.7);
  --accent-color: #2196f3;
  --border-radius: 6px;
  --transition-speed: 0.2s;
}
```

修改这些变量可自定义应用外观。

## 打包部署

### 开发构建

```bash
npm run tauri:dev
```

### 生产构建

```bash
npm run tauri:build
```

构建产物：

- `src-tauri/target/release/desktab.exe` - 可执行文件
- `src-tauri/target/release/bundle/msi/` - MSI 安装包

### 调试模式

```bash
# 启用调试输出
TAURI_DEBUG=1 npm run tauri:build
```

## API 参考

### Tauri Commands

| 命令 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `get_disk_drives` | - | `Vec<String>` | 获取本地磁盘驱动器列表 |
| `get_files_in_directory` | `path: String` | `Vec<FileInfo>` | 获取目录文件列表 |
| `open_file_with_default_app` | `path: String` | `()` | 用默认程序打开文件 |
| `expand_window` | `height: Option<u32>` | `()` | 展开窗口 |
| `collapse_window` | - | `()` | 收缩窗口 |
| `minimize_to_fab` | - | `()` | 最小化为悬浮按钮 |
| `set_always_on_top` | `always_on_top: bool` | `()` | 设置窗口置顶 |
| `toggle_window` | - | `()` | 切换窗口显示/隐藏 |

### FileInfo 结构

```typescript
interface FileInfo {
  path: string;   // 文件完整路径
  name: string;   // 文件名
  size: number;   // 文件大小（字节）
  is_dir: boolean; // 是否为目录
}
```

## 常见问题

### Q: 为什么只支持 Windows？

A: 因为磁盘驱动器检测使用了 Windows API (WinAPI)，如需支持其他平台需要适配对应的系统 API。

### Q: 如何修改窗口默认大小？

A: 修改 `src-tauri/tauri.conf.json` 中的 `width` 和 `height` 配置，以及 `src/components/TopBar.vue` 中的样式高度。

### Q: 如何更改快捷键？

A: 修改 `src-tauri/src/commands/hotkey.rs` 中的快捷键定义。

## 贡献指南

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 代码规范

- 前端代码遵循 Vue 3 Composition API 风格
- 使用 TypeScript 强类型
- Rust 代码遵循标准 Rust 格式规范

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue 3](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Vite](https://vitejs.dev/) - 下一代前端构建工具
- [Pinia](https://pinia.vuejs.org/) - Vue 状态管理