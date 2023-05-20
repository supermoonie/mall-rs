mod cache_service;
mod mem_service;
mod redis_service;

pub use crate::config::config::ApplicationConfig;
pub use cache_service::*;
pub use mem_service::*;
use once_cell::sync::Lazy;
use rbatis::rbatis::Rbatis;
pub use redis_service::*;

/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: Rbatis,
}

impl Default for ServiceContext {
    fn default() -> Self {
        let config = ApplicationConfig::default();
        ServiceContext {
            rb: crate::domain::init_rbatis(&config),
            config,
        }
    }
}
