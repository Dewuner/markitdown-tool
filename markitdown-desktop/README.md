# MarkItDown Desktop v1.1

隐私优先、纯本地运行的跨平台文档转 Markdown 桌面工具。

## 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **UI**: shadcn-vue + Tailwind CSS (Zinc 黑白极简)
- **桌面基座**: Tauri v2 (Rust)
- **转换引擎**: Python (MarkItDown) + uv 管理
- **本地存储**: SQLite (sqlx)

## 项目结构

```
markitdown-desktop/
├── src/                          # Vue 3 前端
│   ├── components/
│   │   ├── layout/               # TopBar, Sidebar, Workspace
│   │   ├── conversion/           # FileDropZone, ConversionProgress
│   │   ├── workspace/            # SplitPaneWorkspace, SourcePanel, MarkdownRenderer
│   │   └── ui/                   # shadcn-vue 组件
│   ├── views/                    # MainView
│   ├── lib/
│   │   ├── composables/          # useAppStore (共享状态)
│   │   ├── types.ts              # TypeScript 类型定义
│   │   ├── tauri.ts              # invoke() 封装
│   │   └── utils.ts              # cn() 工具函数
│   ├── App.vue
│   ├── main.ts
│   └── style.css                 # Tailwind CSS + Zinc 主题变量
├── src-tauri/                    # Rust 后端
│   ├── src/
│   │   ├── lib.rs                # 插件注册 + SQLite 初始化
│   │   ├── commands.rs           # Tauri IPC 命令
│   │   └── models.rs             # 数据模型
│   ├── migrations/               # SQLite 迁移脚本
│   ├── capabilities/             # Tauri 权限配置
│   ├── Cargo.toml
│   └── tauri.conf.json
├── python-core/                  # Python 转换引擎
│   ├── convert.py                # CLI: convert / batch / info
│   ├── requirements.txt
│   └── .venv/                    # uv 管理的虚拟环境
└── package.json
```

## 快速开始

### 环境要求

- Node.js >= 18
- Rust (via rustup)
- Python 3.10+
- [uv](https://docs.astral.sh/uv/) (Python 包管理)
- Tauri 系统依赖 (见下方)

### 安装

```bash
# 1. 安装前端依赖
npm install

# 2. 创建 Python 虚拟环境并安装依赖
cd python-core
uv venv
uv pip install -r requirements.txt
cd ..

# 3. 安装 Tauri 系统依赖 (Linux only)
# Ubuntu/Debian:
sudo apt install -y libwebkit2gtk-4.1-dev build-essential pkg-config \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev \
  patchelf libsoup-3.0-dev libjavascriptcoregtk-4.1-dev

# macOS:
# Xcode Command Line Tools 即可，无需额外安装

# Windows:
# WebView2 (Windows 10/11 已内置)，无需额外安装
```

### 开发

```bash
npm run tauri dev
```

### 打包

```bash
npm run tauri build
```

产出物位于 `src-tauri/target/release/bundle/`。

## IPC 通信

所有跨进程调用遵循统一 JSON 格式：

```json
{ "success": true, "data": { ... } }
{ "success": false, "error": "错误信息" }
```

| 前端 invoke | Rust 命令 | 说明 |
|---|---|---|
| `convertFile({ filePath })` | `convert_file` | 单文件转换 |
| `batchConvert({ filePaths })` | `batch_convert` | 批量转换 |
| `getHistory()` | `get_history` | 获取历史记录 |
| `deleteHistory({ id })` | `delete_history` | 删除记录 |
| `openFileDialog()` | `open_file_dialog` | 打开文件选择器 |
| `openFolderDialog()` | `open_folder_dialog` | 打开文件夹选择器 |

## 支持格式

PDF、DOCX、XLSX、PPTX、HTML、CSV、JSON、XML、TXT、Markdown、图片 (JPG/PNG/GIF/BMP/TIFF)、音频 (WAV/MP3)

## 设计规范

- 黑白极简 (Zinc 色阶: `#fafafa` ~ `#09090b`)
- 字重 + 留白区分层级，不使用多余色彩
- 代码高亮: Shiki (github-light 主题)
- 分隔线: 默认 `#e4e4e7`，悬浮 `#a1a1aa`

## License

MIT
