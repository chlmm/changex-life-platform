# Changex Life Platform

基于 Tauri 2 + Vue 3 + QuickJS 的插件化应用平台

## 项目简介

Changex Life Platform 是一个支持插件的应用平台，允许用户通过编写 JavaScript 插件来扩展应用功能。平台使用 QuickJS 作为 JavaScript 运行时，提供安全的沙箱环境和细粒度的权限控制。

## 核心特性

- **插件系统** - 支持动态加载和执行 JavaScript 插件
- **安全沙箱** - 每个插件运行在独立的隔离环境中
- **权限控制** - 细粒度的权限管理系统（Read, Write, Network, FileSystem）
- **现代 UI** - 基于 Vue 3 + TypeScript 的响应式界面
- **高性能** - 使用 Rust 和 QuickJS 提供卓越性能
- **易于扩展** - 清晰的架构设计，易于开发和部署新插件

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | Vue 3 + TypeScript |
| 构建工具 | Vite |
| 桌面框架 | Tauri 2.0 |
| 后端语言 | Rust |
| JS 运行时 | QuickJS (rquickjs) |
| 异步运行时 | Tokio |

## 项目结构

```
life-platform/
├── web/                           # 前端代码
│   ├── src/
│   │   ├── App.vue                # 插件管理 UI
│   │   └── main.ts                # 应用入口
│   ├── public/                    # 静态资源
│   ├── package.json               # 前端依赖
│   ├── vite.config.ts             # Vite 配置
│   └── tsconfig.json              # TypeScript 配置
│
├── tauri/                         # 后端代码
│   ├── src/
│   │   ├── main.rs                # Tauri 应用入口
│   │   ├── lib.rs                 # 主逻辑 + Tauri 命令
│   │   ├── plugin_api.rs          # 插件 API 和权限系统
│   │   ├── plugin_manager.rs      # 插件管理器
│   │   ├── deno_runtime.rs        # QuickJS 运行时封装
│   │   └── deno_ops.rs            # 文件系统操作实现
│   ├── Cargo.toml                 # Rust 依赖
│   └── tauri.conf.json            # Tauri 配置
│
├── plugins/                       # 示例插件
│   ├── hello-world/               # 基础示例插件
│   │   ├── plugin.json
│   │   └── index.js
│   └── note-manager/              # 笔记管理插件
│       ├── plugin.json
│       └── index.js
│
├── scripts/                       # 项目脚本
│   ├── run.sh                     # 运行脚本
│   ├── start-novnc.sh             # 启动 noVNC
│   └── stop-novnc.sh              # 停止 noVNC
│
├── doc/                           # 项目文档
│   ├── REFERENCE.md               # 快速参考
│   ├── PROJECT_STATUS.md          # 项目状态
│   ├── FINAL_STATUS.md            # 最终状态
│   ├── RECOVERY_REPORT.md         # 恢复报告
│   ├── PROJECT_SUMMARY_COMPLETE.md # 项目总结
│   └── COMPLETION_REPORT.md       # 完成报告
│
└── .github/                       # CI/CD 配置
    └── workflows/build.yml
```

## 快速开始

### 环境要求

- Ubuntu 22.04+ 或 Docker 环境
- Rust 1.78.0+
- Node.js 18.x+
- webkit2gtk-4.1-dev

### 安装依赖

```bash
# 安装系统依赖 (Ubuntu 22.04+)
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    pkg-config

# 安装前端依赖
cd web && npm install
```

### 开发运行

```bash
# 方式一：使用 npm 脚本（推荐）
cd web && npm run tauri dev

# 方式二：手动启动
cd web && npm run dev        # 终端1：启动前端
cd tauri && cargo tauri dev  # 终端2：启动 Tauri
```

### 生产构建

```bash
cd web && npm run tauri build
```

## 插件开发

### 插件目录结构

```
my-plugin/
├── plugin.json      # 必需：插件元数据
├── index.js         # 必需：插件入口文件
└── data/            # 自动创建：插件数据目录
```

### plugin.json 格式

```json
{
  "name": "my-plugin",
  "version": "1.0.0",
  "description": "我的插件描述",
  "author": "作者名称",
  "permissions": ["FileSystem", "Read", "Write"],
  "entry_point": "index.js"
}
```

### 插件代码示例

```javascript
// index.js
export default {
  async execute() {
    // 获取插件信息
    const info = getPluginInfo();
    console.log(`Running ${info.name} v${info.version}`);

    // 读取文件
    const content = readFile("greeting.txt");
    console.log(content);

    // 写入文件
    writeFile("output.txt", "Hello from plugin!");

    // 列出目录
    const files = listDir(".");
    console.log("Files:", files);

    return "Plugin executed successfully!";
  }
};
```

### 安装插件

```bash
# 创建插件目录
mkdir -p ~/.life-platform/plugins

# 复制插件到插件目录
cp -r plugins/hello-world ~/.life-platform/plugins/
```

## API 参考

### Tauri 命令（后端 API）

| 命令 | 说明 | 返回值 |
|------|------|--------|
| `list_plugins()` | 列出所有已安装的插件 | `string[]` |
| `execute_plugin(pluginName)` | 执行指定插件 | `PluginResult` |
| `get_plugin_metadata(pluginName)` | 获取插件元数据 | `PluginMetadata` |
| `get_plugins_dir()` | 获取插件目录路径 | `string` |

### 插件 API（JS 运行时）

| API | 说明 |
|-----|------|
| `getPluginInfo()` | 获取当前插件信息 |
| `readFile(path)` | 读取文件内容 |
| `writeFile(path, content)` | 写入文件内容 |
| `listDir(path)` | 列出目录内容 |
| `console.log(msg)` | 日志输出 |

## 权限系统

插件在 `plugin.json` 中声明所需的权限：

| 权限 | 说明 |
|------|------|
| `Read` | 读取文件权限 |
| `Write` | 写入文件权限 |
| `Network` | 网络访问权限（预留） |
| `FileSystem` | 文件系统操作权限 |

### 安全特性

- **沙箱隔离** - 每个插件运行在独立的 QuickJS 实例中
- **路径遍历防护** - 防止访问插件目录外的文件
- **细粒度权限** - 精确控制每个插件的访问权限

## 常用命令

```bash
# 前端开发
cd web && npm run dev          # 启动开发服务器
cd web && npm run build        # 构建前端
cd web && npm run preview      # 预览构建结果

# 后端开发
cd tauri && cargo check        # 检查 Rust 代码
cd tauri && cargo build        # 构建后端
cd tauri && cargo test         # 运行测试
cd tauri && cargo fmt          # 格式化代码

# Tauri 开发
cd web && npm run tauri dev    # 开发模式
cd web && npm run tauri build  # 生产构建
```

## 项目状态

### 已完成

- 完整的代码实现（900+ 行）
- Tauri 2 + Vue 3 框架
- QuickJS 运行时集成
- 插件系统核心模块
- 权限系统和沙箱机制
- Vue 插件管理 UI
- 示例插件（hello-world, note-manager）
- 完整文档

### 待开发

- 网络权限实现
- 插件热重载
- 插件市场
- 更多系统 API

## 文档

| 文档 | 说明 |
|------|------|
| [REFERENCE.md](./doc/REFERENCE.md) | 快速参考和 API 文档 |
| [PROJECT_STATUS.md](./doc/PROJECT_STATUS.md) | 项目状态报告 |
| [FINAL_STATUS.md](./doc/FINAL_STATUS.md) | 最终状态和解决方案 |
| [RECOVERY_REPORT.md](./doc/RECOVERY_REPORT.md) | 项目恢复报告 |
| [PROJECT_SUMMARY_COMPLETE.md](./doc/PROJECT_SUMMARY_COMPLETE.md) | 完整项目总结 |
| [COMPLETION_REPORT.md](./doc/COMPLETION_REPORT.md) | 项目交付清单 |

## 许可证

MIT License

---

**项目名称**: Life Platform（人生平台）  
**版本**: MVP 0.1.0  
**状态**: 代码完成，需要 Ubuntu 22.04+ 或 Docker 环境
