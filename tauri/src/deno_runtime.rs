use rquickjs::Runtime;
use std::fs;
use std::path::PathBuf;

use super::deno_ops::execute_plugin_sync;
use super::plugin_api::{PluginMetadata, PluginResult};

/// QuickJS 运行时管理器
pub struct JsPluginRuntime {
    runtime: Runtime,
}

impl JsPluginRuntime {
    /// 创建新的 JS 运行时
    pub fn new() -> Result<Self, String> {
        let runtime = Runtime::new().map_err(|e| format!("Failed to create runtime: {}", e))?;
        Ok(JsPluginRuntime { runtime })
    }

    /// 加载并执行插件文件
    pub fn execute_plugin(
        &self,
        metadata: &PluginMetadata,
        base_path: PathBuf,
        plugin_path: PathBuf,
    ) -> Result<PluginResult, String> {
        // 读取插件源代码
        let code = fs::read_to_string(&plugin_path)
            .map_err(|e| format!("Failed to read plugin file: {}", e))?;

        // 执行插件
        Ok(execute_plugin_sync(&self.runtime, metadata, base_path, &code))
    }
}
