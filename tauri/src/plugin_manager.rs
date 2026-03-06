use std::fs;
use std::path::{Path, PathBuf};
use dirs::home_dir;

use super::plugin_api::{PluginMetadata, PluginResult, PluginContext};
use super::deno_runtime::JsPluginRuntime;

/// 插件管理器
pub struct PluginManager {
    plugins_dir: PathBuf,
}

impl PluginManager {
    /// 创建新的插件管理器
    pub fn new() -> Result<Self, String> {
        // 在用户目录下创建插件目录
        let home = home_dir().ok_or("Failed to get home directory")?;
        let plugins_dir = home.join(".life-platform").join("plugins");

        // 确保目录存在
        fs::create_dir_all(&plugins_dir)
            .map_err(|e| format!("Failed to create plugins directory: {}", e))?;

        Ok(PluginManager { plugins_dir })
    }

    /// 获取插件目录
    pub fn plugins_dir(&self) -> &Path {
        &self.plugins_dir
    }

    /// 加载插件元数据
    pub fn load_metadata(&self, plugin_dir: &Path) -> Result<PluginMetadata, String> {
        let metadata_path = plugin_dir.join("plugin.json");

        let content = fs::read_to_string(&metadata_path)
            .map_err(|e| format!("Failed to read plugin.json: {}", e))?;

        let metadata: PluginMetadata = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse plugin.json: {}", e))?;

        Ok(metadata)
    }

    /// 创建插件执行上下文
    pub fn create_context(&self, metadata: &PluginMetadata) -> Result<PluginContext, String> {
        let plugin_dir = self.plugins_dir.join(&metadata.name);
        let base_path = plugin_dir.join("data");

        // 创建数据目录
        fs::create_dir_all(&base_path)
            .map_err(|e| format!("Failed to create plugin data directory: {}", e))?;

        Ok(PluginContext {
            plugin_id: metadata.name.clone(),
            metadata: metadata.clone(),
            permissions: metadata.permissions.clone(),
            base_path,
        })
    }

    /// 执行插件
    pub fn execute_plugin(&self, plugin_name: &str) -> Result<PluginResult, String> {
        // 加载插件元数据
        let plugin_dir = self.plugins_dir.join(plugin_name);
        let metadata = self.load_metadata(&plugin_dir)?;

        // 创建上下文（确保数据目录存在）
        let context = self.create_context(&metadata)?;
        let base_path = context.base_path;

        // 创建 JS 运行时
        let runtime = JsPluginRuntime::new()?;

        // 执行插件
        let entry_point = plugin_dir.join(&metadata.entry_point);
        runtime.execute_plugin(&metadata, base_path, entry_point)
    }

    /// 列出所有已安装的插件
    pub fn list_plugins(&self) -> Result<Vec<String>, String> {
        let mut plugins = Vec::new();

        let entries = fs::read_dir(&self.plugins_dir)
            .map_err(|e| format!("Failed to read plugins directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                let metadata_path = path.join("plugin.json");
                if metadata_path.exists() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        plugins.push(name.to_string());
                    }
                }
            }
        }

        Ok(plugins)
    }

    /// 安装插件（复制插件文件到插件目录）
    pub fn install_plugin(&self, plugin_dir: &Path) -> Result<String, String> {
        let metadata_path = plugin_dir.join("plugin.json");

        if !metadata_path.exists() {
            return Err("plugin.json not found in plugin directory".to_string());
        }

        // 读取元数据
        let content = fs::read_to_string(&metadata_path)
            .map_err(|e| format!("Failed to read plugin.json: {}", e))?;
        let metadata: PluginMetadata = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse plugin.json: {}", e))?;

        // 目标目录
        let target_dir = self.plugins_dir.join(&metadata.name);

        // 创建目标目录
        fs::create_dir_all(&target_dir)
            .map_err(|e| format!("Failed to create target directory: {}", e))?;

        // 复制所有文件
        for entry in fs::read_dir(plugin_dir)
            .map_err(|e| format!("Failed to read source plugin directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let source = entry.path();
            let target = target_dir.join(
                source.file_name()
                    .ok_or("Invalid file name".to_string())?
            );

            if source.is_file() {
                fs::copy(&source, &target)
                    .map_err(|e| format!("Failed to copy file: {}", e))?;
            }
        }

        Ok(format!("Plugin '{}' installed successfully", metadata.name))
    }
}
