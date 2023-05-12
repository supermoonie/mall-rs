pub use crate::config::config::ApplicationConfig;
use once_cell::sync::Lazy;
use rbatis::rbatis::Rbatis;

/// CONTEXT is all of the service struct
pub static CONTEXT: Lazy<ServiceContext> = Lazy::new(|| ServiceContext::default());

pub struct ServiceContext {
    pub config: ApplicationConfig,
    pub rb: Rbatis,
}
