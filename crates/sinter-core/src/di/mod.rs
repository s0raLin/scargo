//! 依赖注入模块
//!
//! 提供服务容器和依赖注入功能，支持服务的注册和解析

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 服务容器
///
/// 管理所有服务的注册和解析，支持单例模式
pub struct ServiceContainer {
    services: RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}

impl ServiceContainer {
    /// 创建新的服务容器
    pub fn new() -> Self {
        Self {
            services: RwLock::new(HashMap::new()),
        }
    }

    /// 注册单例服务
    ///
    /// 服务必须实现 Send + Sync trait
    pub fn register_singleton<T: 'static + Send + Sync>(
        &self,
        service: T,
    ) -> Result<(), String> {
        let type_id = TypeId::of::<T>();
        let mut services = self.services.write().map_err(|e| e.to_string())?;
        services.insert(type_id, Box::new(Arc::new(service)));
        Ok(())
    }

    /// 注册工厂函数
    ///
    /// 每次解析都会调用工厂函数创建新的实例
    pub fn register_factory<T: 'static + Send + Sync, F>(
        &self,
        factory: F,
    ) -> Result<(), String>
    where
        F: Fn() -> T + 'static + Send + Sync,
    {
        let type_id = TypeId::of::<T>();
        let mut services = self.services.write().map_err(|e| e.to_string())?;
        services.insert(type_id, Box::new(Arc::new(factory)));
        Ok(())
    }

    /// 解析服务
    ///
    /// 返回服务的 Arc 引用，支持多线程安全访问
    pub fn resolve<T: 'static + Send + Sync>(&self) -> Result<Arc<T>, String> {
        let type_id = TypeId::of::<T>();
        let services = self.services.read().map_err(|e| e.to_string())?;

        let service = services
            .get(&type_id)
            .ok_or_else(|| format!("Service not registered: {:?}", type_id))?;

        // Try to downcast to Arc<T>
        if let Some(singleton) = service.downcast_ref::<Arc<T>>() {
            Ok(Arc::clone(singleton))
        } else if let Some(factory) = service.downcast_ref::<Arc<dyn Fn() -> T + Send + Sync>>() {
            Ok(Arc::new(factory()))
        } else {
            Err(format!("Service type mismatch for: {:?}", type_id))
        }
    }

    /// 检查服务是否已注册
    pub fn is_registered<T: 'static>(&self) -> bool {
        let type_id = TypeId::of::<T>();
        self.services.read().map(|s| s.contains_key(&type_id)).unwrap_or(false)
    }
}

impl Default for ServiceContainer {
    fn default() -> Self {
        Self::new()
    }
}

/// 服务提供者 trait
///
/// 定义如何创建和提供服务
pub trait ServiceProvider: Send + Sync {
    /// 注册所有服务到容器中
    fn register_services(&self, container: &ServiceContainer) -> Result<(), String>;
}

/// 默认服务提供者
///
/// 注册所有核心服务
pub struct DefaultServiceProvider;

impl DefaultServiceProvider {
    pub fn new() -> Self {
        Self
    }
}

impl ServiceProvider for DefaultServiceProvider {
    fn register_services(&self, container: &ServiceContainer) -> Result<(), String> {
        // 注册项目服务
        use crate::services::project::ProjectServiceImpl;
        container.register_singleton(ProjectServiceImpl::new())?;

        Ok(())
    }
}

impl Default for DefaultServiceProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// 依赖注入上下文
///
/// 持有服务容器，提供便捷的服务解析方法
pub struct DIContext {
    container: Arc<ServiceContainer>,
}

impl DIContext {
    /// 创建新的依赖注入上下文
    pub fn new(container: Arc<ServiceContainer>) -> Self {
        Self { container }
    }

    /// 获取服务容器的引用
    pub fn container(&self) -> &ServiceContainer {
        &self.container
    }

    /// 解析服务
    pub fn resolve<T: 'static + Send + Sync>(&self) -> Result<Arc<T>, String> {
        self.container.resolve()
    }
}

impl Clone for DIContext {
    fn clone(&self) -> Self {
        Self {
            container: Arc::clone(&self.container),
        }
    }
}

/// 全局服务容器实例
static mut GLOBAL_CONTAINER: Option<Arc<ServiceContainer>> = None;

/// 初始化全局服务容器
pub fn init_global_container(provider: &dyn ServiceProvider) -> Result<(), String> {
    unsafe {
        if GLOBAL_CONTAINER.is_some() {
            return Err("Global container already initialized".to_string());
        }

        let container = Arc::new(ServiceContainer::new());
        provider.register_services(&container)?;
        GLOBAL_CONTAINER = Some(Arc::clone(&container));
        Ok(())
    }
}

/// 获取全局服务容器
pub fn get_global_container() -> Result<Arc<ServiceContainer>, String> {
    unsafe {
        GLOBAL_CONTAINER
            .as_ref()
            .map(Arc::clone)
            .ok_or_else(|| "Global container not initialized".to_string())
    }
}

/// 获取全局依赖注入上下文
pub fn get_global_context() -> Result<DIContext, String> {
    get_global_container().map(DIContext::new)
}