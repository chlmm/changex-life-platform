mod plugin_api;
mod deno_ops;
mod deno_runtime;
mod plugin_manager;

use plugin_manager::PluginManager;
use std::sync::Mutex;
use tauri::State;

// 应用状态，包含插件管理器
struct AppState {
    plugin_manager: Mutex<PluginManager>,
}

/// 问候命令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

/// 列出所有插件
#[tauri::command]
fn list_plugins(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let manager = state.plugin_manager.lock().unwrap();
    manager.list_plugins()
}

/// 执行插件
#[tauri::command]
fn execute_plugin(
    plugin_name: String,
    state: State<'_, AppState>
) -> Result<plugin_api::PluginResult, String> {
    let manager = state.plugin_manager.lock().unwrap();
    manager.execute_plugin(&plugin_name)
}

/// 获取插件元数据
#[tauri::command]
fn get_plugin_metadata(
    plugin_name: String,
    state: State<'_, AppState>
) -> Result<plugin_api::PluginMetadata, String> {
    let manager = state.plugin_manager.lock().unwrap();
    let plugin_dir = manager.plugins_dir().join(&plugin_name);
    manager.load_metadata(&plugin_dir)
}

/// 获取插件目录路径
#[tauri::command]
fn get_plugins_dir(state: State<'_, AppState>) -> Result<String, String> {
    let manager = state.plugin_manager.lock().unwrap();
    manager.plugins_dir()
        .to_str()
        .map(|s| s.to_string())
        .ok_or("Failed to convert path to string".to_string())
}

pub fn run() {
    // 初始化插件管理器
    let plugin_manager = PluginManager::new()
        .expect("Failed to initialize plugin manager");

    tauri::Builder::default()
        .manage(AppState {
            plugin_manager: Mutex::new(plugin_manager),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            list_plugins,
            execute_plugin,
            get_plugin_metadata,
            get_plugins_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
