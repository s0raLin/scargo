use std::collections::HashMap;
use std::path::PathBuf;
use libloading::{Library, Symbol};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

/// 插件接口定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub commands: Vec<String>,
}

/// 插件管理器
pub struct PluginManager {
    plugins: HashMap<String, PluginHandle>,
    plugin_dir: PathBuf,
}

struct PluginHandle {
    _lib: Option<Library>, // 使用Option来允许移动
    info: PluginInfo,
    execute_fn: Symbol<'static, unsafe extern "C" fn(args: *const std::ffi::c_char) -> *mut std::ffi::c_char>,
}

impl PluginManager {
    pub fn new() -> Result<Self> {
        let plugin_dir = dirs::home_dir()
            .context("Could not find home directory")?
            .join(".scargo")
            .join("plugins");

        std::fs::create_dir_all(&plugin_dir)?;

        Ok(Self {
            plugins: HashMap::new(),
            plugin_dir,
        })
    }

    /// 加载所有插件
    pub fn load_plugins(&mut self) -> Result<()> {
        for entry in std::fs::read_dir(&self.plugin_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("so") ||
               path.extension().and_then(|s| s.to_str()) == Some("dylib") ||
               path.extension().and_then(|s| s.to_str()) == Some("dll") {
                if let Err(e) = self.load_plugin(&path) {
                    eprintln!("Failed to load plugin {}: {}", path.display(), e);
                }
            }
        }
        Ok(())
    }

    /// 加载单个插件
    fn load_plugin(&mut self, path: &std::path::Path) -> Result<()> {
        unsafe {
            let lib = Library::new(path)?;
            let get_info: Symbol<unsafe extern "C" fn() -> *mut std::ffi::c_char> =
                lib.get(b"scargo_plugin_info")?;
            let execute: Symbol<unsafe extern "C" fn(args: *const std::ffi::c_char) -> *mut std::ffi::c_char> =
                lib.get(b"scargo_plugin_execute")?;

            let info_ptr = get_info();
            let info_json = std::ffi::CStr::from_ptr(info_ptr).to_str()?;
            let info: PluginInfo = serde_json::from_str(info_json)?;

            // 释放插件分配的内存
            libc::free(info_ptr as *mut libc::c_void);

            // 转义函数指针到静态生命周期
            let execute_fn: Symbol<'static, unsafe extern "C" fn(args: *const std::ffi::c_char) -> *mut std::ffi::c_char> =
                std::mem::transmute(execute);

            let handle = PluginHandle {
                _lib: Some(lib),
                info: info.clone(),
                execute_fn,
            };

            self.plugins.insert(info.name.clone(), handle);
        }

        Ok(())
    }

    /// 执行插件命令
    pub fn execute_plugin(&self, name: &str, args: &[String]) -> Result<String> {
        let plugin = self.plugins.get(name)
            .context(crate::t!("error.plugin_not_found", &[name]))?;

        let args_json = serde_json::to_string(args)?;
        let args_cstr = std::ffi::CString::new(args_json)?;

        unsafe {
            let result_ptr = (plugin.execute_fn)(args_cstr.as_ptr());
            let result_cstr = std::ffi::CStr::from_ptr(result_ptr);
            let result = result_cstr.to_str()?.to_string();

            // 释放插件分配的内存
            libc::free(result_ptr as *mut libc::c_void);

            Ok(result)
        }
    }

    /// 获取所有插件信息
    pub fn list_plugins(&self) -> Vec<&PluginInfo> {
        self.plugins.values().map(|h| &h.info).collect()
    }

    /// 检查插件是否存在
    pub fn has_plugin(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }
}

/// 插件API - 用于插件实现
pub mod api {
    use super::*;

    /// 插件执行结果
    pub enum PluginResult {
        Success(String),
        Error(String),
    }

    /// 插件接口trait
    pub trait Plugin {
        fn info(&self) -> PluginInfo;
        fn execute(&self, command: &str, args: Vec<String>) -> PluginResult;
    }

    /// 导出函数包装器
    #[macro_export]
    macro_rules! export_plugin {
        ($plugin:expr) => {
            use std::ffi::CString;
            use std::os::raw::c_char;

            #[no_mangle]
            pub extern "C" fn scargo_plugin_info() -> *mut c_char {
                let info = $plugin.info();
                let json = serde_json::to_string(&info).unwrap();
                CString::new(json).unwrap().into_raw()
            }

            #[no_mangle]
            pub extern "C" fn scargo_plugin_execute(args: *const c_char) -> *mut c_char {
                let args_cstr = unsafe { std::ffi::CStr::from_ptr(args) };
                let args_json = args_cstr.to_str().unwrap();
                let args: Vec<String> = serde_json::from_str(args_json).unwrap();

                if args.is_empty() {
                    let error = PluginResult::Error("No command specified".to_string());
                    let json = serde_json::to_string(&error).unwrap();
                    return CString::new(json).unwrap().into_raw();
                }

                let command = &args[0];
                let command_args = args[1..].to_vec();

                let result = $plugin.execute(command, command_args);
                let json = serde_json::to_string(&result).unwrap();
                CString::new(json).unwrap().into_raw()
            }
        };
    }
}