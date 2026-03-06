use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 插件权限定义
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Network,
    FileSystem,
}

/// 插件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub permissions: Vec<Permission>,
    pub entry_point: String,
}

/// 插件执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

/// 插件执行上下文
#[derive(Clone)]
pub struct PluginContext {
    pub plugin_id: String,
    pub metadata: PluginMetadata,
    pub permissions: Vec<Permission>,
    pub base_path: PathBuf,
}

/// 检查插件是否具有特定权限
pub fn check_permission(context: &PluginContext, permission: Permission) -> bool {
    context.permissions.contains(&permission)
}
