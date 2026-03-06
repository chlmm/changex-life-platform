use rquickjs::{Ctx, Runtime, Function};
use std::path::PathBuf;
use std::fs;

use super::plugin_api::{Permission, PluginMetadata};

/// 执行插件的同步版本
pub fn execute_plugin_sync(
    runtime: &Runtime,
    metadata: &PluginMetadata,
    base_path: PathBuf,
    code: &str,
) -> crate::plugin_api::PluginResult {
    let permissions = metadata.permissions.clone();
    let base_path_str = base_path.to_string_lossy().to_string();
    
    let ctx = match rquickjs::Context::full(runtime) {
        Ok(ctx) => ctx,
        Err(e) => {
            return crate::plugin_api::PluginResult {
                success: false,
                output: String::new(),
                error: Some(format!("Failed to create context: {}", e)),
            }
        }
    };

    let result = ctx.with(|ctx| {
        // 注入插件 API 到全局对象（使用 Rust 原生实现）
        if let Err(e) = inject_plugin_api(&ctx, &base_path_str, &permissions) {
            return crate::plugin_api::PluginResult {
                success: false,
                output: String::new(),
                error: Some(format!("Failed to inject API: {:?}", e)),
            };
        }

        // 执行插件代码
        if let Err(e) = ctx.eval::<(), _>(code) {
            return crate::plugin_api::PluginResult {
                success: false,
                output: String::new(),
                error: Some(format!("Execution error: {:?}", e)),
            };
        }

        // 尝试调用 main 函数
        let main_result: Result<String, _> = ctx.eval(
            r#"
                (function() {
                    if (typeof main !== 'undefined' && typeof main === 'function') {
                        return String(main());
                    }
                    return "Plugin loaded successfully";
                })()
            "#,
        );

        match main_result {
            Ok(output) => crate::plugin_api::PluginResult {
                success: true,
                output,
                error: None,
            },
            Err(_) => crate::plugin_api::PluginResult {
                success: true,
                output: "Plugin loaded but no output".to_string(),
                error: None,
            },
        }
    });

    result
}

/// 创建 JS 错误
fn js_err(msg: &str) -> rquickjs::Error {
    rquickjs::Error::new_from_js_message("Rust", "JS", msg)
}

/// 注入插件 API 到 JS 上下文
fn inject_plugin_api(
    ctx: &Ctx,
    base_path: &str,
    permissions: &[Permission],
) -> rquickjs::Result<()> {
    let can_read = permissions.contains(&Permission::Read);
    let can_write = permissions.contains(&Permission::Write);
    let can_list = permissions.contains(&Permission::FileSystem);
    
    let global = ctx.globals();
    
    // 注册 console.log
    let log_func = Function::new(ctx.clone(), |msg: String| {
        println!("[Plugin] {}", msg);
    })?;
    global.set("__log", log_func)?;
    
    let console_code = r#"
        var console = {
            log: function(msg) { __log(String(msg)); }
        };
    "#;
    ctx.eval::<(), _>(console_code)?;
    
    // 注册 read_file
    let base_path_read = base_path.to_string();
    let read_func = Function::new(ctx.clone(), move |path: String| -> Result<String, rquickjs::Error> {
        if !can_read {
            return Err(js_err("Read permission denied"));
        }
        let full_path = PathBuf::from(&base_path_read).join(&path);
        fs::read_to_string(&full_path)
            .map_err(|e| js_err(&format!("Failed to read file: {}", e)))
    })?;
    global.set("__read_file_impl", read_func)?;
    
    let read_code = r#"
        function read_file(path) {
            return __read_file_impl(path);
        }
    "#;
    ctx.eval::<(), _>(read_code)?;
    
    // 注册 write_file
    let base_path_write = base_path.to_string();
    let write_func = Function::new(ctx.clone(), move |path: String, content: String| -> Result<(), rquickjs::Error> {
        if !can_write {
            return Err(js_err("Write permission denied"));
        }
        let full_path = PathBuf::from(&base_path_write).join(&path);
        if let Some(parent) = full_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| js_err(&format!("Failed to create directory: {}", e)))?;
            }
        }
        fs::write(&full_path, content)
            .map_err(|e| js_err(&format!("Failed to write file: {}", e)))
    })?;
    global.set("__write_file_impl", write_func)?;
    
    let write_code = r#"
        function write_file(path, content) {
            __write_file_impl(path, content);
        }
    "#;
    ctx.eval::<(), _>(write_code)?;
    
    // 注册 list_dir - 返回 Vec<String>，由 rquickjs 自动转换为 JS 数组
    let base_path_list = base_path.to_string();
    let list_func = Function::new(ctx.clone(), move |path: String| -> Result<Vec<String>, rquickjs::Error> {
        if !can_list {
            return Err(js_err("FileSystem permission denied"));
        }
        let full_path = PathBuf::from(&base_path_list).join(&path);
        let entries = fs::read_dir(&full_path)
            .map_err(|e| js_err(&format!("Failed to list directory: {}", e)))?;
        
        let mut result = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                let name: String = entry.file_name().to_string_lossy().to_string();
                result.push(name);
            }
        }
        Ok(result)
    })?;
    global.set("__list_dir_impl", list_func)?;
    
    let list_code = r#"
        function list_dir(path) {
            return __list_dir_impl(path);
        }
    "#;
    ctx.eval::<(), _>(list_code)?;
    
    // 注册 get_plugin_info
    let info_code = r#"
        function get_plugin_info() {
            return {
                name: "plugin",
                version: "1.0.0",
                author: "unknown"
            };
        }
    "#;
    ctx.eval::<(), _>(info_code)?;
    
    Ok(())
}
