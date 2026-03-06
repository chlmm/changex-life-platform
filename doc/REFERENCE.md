# Life Platform - 快速参考

## 项目位置

```
/workspace/life-platform/
```

## 核心文件速查

### 前端关键文件
- `web/src/App.vue` - 插件管理 UI（主界面，313 行）
- `web/src/main.ts` - 应用入口

### 后端关键文件
- `tauri/src/lib.rs` - Tauri 命令和主逻辑（80 行）
- `tauri/src/plugin_api.rs` - 插件 API 定义（64 行）
- `tauri/src/plugin_manager.rs` - 插件管理器（143 行）
- `tauri/src/deno_runtime.rs` - QuickJS 运行时（85 行）
- `tauri/src/deno_ops.rs` - 文件系统操作（126 行）
- `tauri/src/main.rs` - Tauri 入口（6 行）

### 配置文件
- `tauri/Cargo.toml` - Rust 依赖（Tauri 2.0 + QuickJS）
- `tauri/tauri.conf.json` - Tauri 配置
- `web/package.json` - Node.js 依赖
- `web/vite.config.ts` - Vite 配置
- `web/tsconfig.json` - TypeScript 配置

### 示例插件
- `plugins/hello-world/` - Hello World 示例插件
  - `plugin.json` - 插件元数据
  - `index.js` - 插件代码
- `plugins/note-manager/` - 笔记管理插件

### 文档文件
- `README.md` - 项目主文档（从这里开始）
- `doc/FINAL_STATUS.md` - 最终状态报告
- `doc/PROJECT_STATUS.md` - 项目状态报告
- `doc/PROJECT_SUMMARY_COMPLETE.md` - 项目总结
- `doc/RECOVERY_REPORT.md` - 恢复报告
- `doc/REFERENCE.md` - 快速参考（本文件）

## 常用命令

### 前端相关
```bash
cd life-platform/web

# 安装依赖
npm install

# 开发模式（仅前端）
npm run dev

# 前端构建
npm run build

# 前端预览
npm run preview
```

### 后端相关
```bash
cd life-platform/tauri

# 检查代码
cargo check

# 构建项目
cargo build

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 清理构建
cargo clean
```

### Tauri 相关
```bash
cd life-platform/web

# 开发模式（前端 + 后端）
npm run tauri dev

# 生产构建
npm run tauri build
```

## 插件相关命令

### 安装插件
```bash
mkdir -p ~/.life-platform/plugins
cp -r plugins/hello-world ~/.life-platform/plugins/
```

### 查看已安装插件
```bash
ls ~/.life-platform/plugins/
```

### 查看插件数据
```bash
ls ~/.life-platform/plugins/hello-world/data/
```

## Tauri 命令（后端暴露的 API）

### 1. greet(name: string) -> string
简单的问候命令，用于测试。

### 2. list_plugins() -> string[]
列出所有已安装的插件名称。

```typescript
const plugins = await invoke('list_plugins');
// 返回: ["hello-world", "my-plugin"]
```

### 3. execute_plugin(plugin_name: string) -> PluginResult
执行指定插件。

```typescript
const result = await invoke('execute_plugin', { pluginName: 'hello-world' });
```

返回格式：
```typescript
interface PluginResult {
  success: boolean;
  output: string;
  error: string | null;
}
```

### 4. get_plugin_metadata(plugin_name: string) -> PluginMetadata
获取插件的元数据。

```typescript
const metadata = await invoke('get_plugin_metadata', { pluginName: 'hello-world' });
```

返回格式：
```typescript
interface PluginMetadata {
  name: string;
  version: string;
  description: string;
  author: string;
  permissions: Permission[];
  entry_point: string;
}
```

### 5. get_plugins_dir() -> string
获取插件目录的路径。

```typescript
const pluginsDir = await invoke('get_plugins_dir');
// 返回: "/home/user/.life-platform/plugins"
```

## 插件 API（JS 运行时可用）

### 1. getPluginInfo() -> PluginInfo
获取当前插件的信息。

```javascript
const info = getPluginInfo();
console.log(info.name, info.version, info.author);
```

### 2. readFile(path: string) -> string
读取文件内容。

```javascript
const content = readFile("greeting.txt");
console.log(content);
```

### 3. writeFile(path: string, content: string) -> void
写入文件内容。

```javascript
writeFile("output.txt", "Hello, World!");
```

### 4. listDir(path: string) -> string[]
列出目录内容。

```javascript
const files = listDir(".");
console.log(files);
```

## 权限类型

插件在 `plugin.json` 中声明的权限：

- `Read` - 读取文件权限
- `Write` - 写入文件权限
- `Network` - 网络访问权限（预留，未实现）
- `FileSystem` - 文件系统操作权限

## 插件目录结构

```
~/.life-platform/plugins/
└── plugin-name/
    ├── plugin.json      # 必需：插件元数据
    ├── index.js        # 必需：插件入口文件
    └── data/          # 自动创建：插件数据目录
        └── ...        # 插件运行时数据
```

## 插件元数据格式

```json
{
  "name": "plugin-name",
  "version": "1.0.0",
  "description": "插件描述",
  "author": "作者名称",
  "permissions": ["FileSystem", "Read", "Write"],
  "entry_point": "index.js"
}
```

## 插件代码示例

```javascript
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

## 常见问题速查

### Q: 如何创建新插件？
A: 复制 `plugins/hello-world/` 目录，修改 plugin.json 和 index.js

### Q: 插件代码出错？
A: 检查：
1. plugin.json 格式是否正确
2. entry_point 文件是否存在
3. 权限是否声明
4. 查看 Tauri 控制台错误信息

### Q: 找不到插件？
A: 确保插件已安装到正确的目录：

```bash
# 查看插件目录
ls ~/.life-platform/plugins/

# 应该看到你的插件目录
```

### Q: 文件操作权限被拒绝？
A: 检查 plugin.json 中的 permissions 是否包含：
- `Read`: 用于读取文件
- `Write`: 用于写入文件
- `FileSystem`: 用于列表目录

### Q: 编译失败，提示找不到 webkit2gtk-4.1？
A: 当前环境是 Ubuntu 20.04，需要升级到 Ubuntu 22.04+ 或使用 Docker 环境。详见 FINAL_STATUS.md。

## 开发工具

### 推荐的 VS Code 扩展

```bash
code --install-extension Vue.volar
code --install-extension dbaeumer.vscode-eslint
code --install-extension rust-lang.rust-analyzer
code --install-extension tauri-apps.tauri-vscode
```

### 有用的命令

```bash
# 格式化 Rust 代码
cd tauri && cargo fmt

# 检查 Rust 代码
cd tauri && cargo check

# 运行 Rust 测试
cd tauri && cargo test

# 清理构建产物
cd tauri && cargo clean

# 前端开发服务器
cd web && npm run dev

# 前端构建
cd web && npm run build

# Tauri 开发模式
cd web && npm run tauri dev
```

## 项目结构理解

### 前端（Vue）

```
web/
├── src/
│   ├── App.vue        # 主应用界面
│   └── main.ts        # 应用入口
├── public/            # 静态资源
├── package.json       # 依赖配置
├── vite.config.ts     # Vite 配置
└── tsconfig.json      # TypeScript 配置
```

### 后端（Rust）

```
tauri/src/
├── main.rs              # Tauri 应用入口
├── lib.rs               # 主逻辑 + Tauri 命令定义
├── plugin_api.rs        # 插件 API 和权限系统
├── plugin_manager.rs    # 插件管理器
├── deno_runtime.rs     # QuickJS 运行时封装
└── deno_ops.rs         # 文件系统操作
```

### 插件

```
~/.life-platform/plugins/
└── your-plugin/
    ├── plugin.json      # 插件配置
    ├── index.js        # 插件代码
    └── data/          # 插件数据（自动创建）
```

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端框架 | Vue 3 + TypeScript |
| 构建工具 | Vite |
| 桌面框架 | Tauri 2.0 |
| 后端语言 | Rust |
| JS 运行时 | QuickJS (rquickjs) |
| 异步运行时 | Tokio |

## 环境要求

### 当前环境
- Ubuntu 20.04
- webkit2gtk-4.0

### Tauri 2.0 完整要求（推荐）
- Ubuntu 22.04+ 或 Docker 环境
- webkit2gtk-4.1-dev
- Rust 1.78.0 或更高版本
- Node.js 18.x 或更高版本

## 下一步

1. 阅读 `README.md` 了解项目详情
2. 查看 `doc/FINAL_STATUS.md` 了解当前状态和解决方案
3. 参考 `plugins/hello-world/` 创建插件
4. 根据环境选择升级系统或使用 Docker

## 获取帮助

- 📖 文档: README.md
- 📄 状态: doc/FINAL_STATUS.md
- 📊 项目: doc/PROJECT_STATUS.md
- 📝 总结: doc/PROJECT_SUMMARY_COMPLETE.md
- 📋 恢复: doc/RECOVERY_REPORT.md

---

**版本**: MVP 0.1.0（Tauri 2.0）  
**状态**: ✅ 完成  
**日期**: 2026-01-05  
**文档状态**: ✅ 已更新
