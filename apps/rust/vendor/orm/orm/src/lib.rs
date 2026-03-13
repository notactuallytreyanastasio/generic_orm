#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
pub mod src;
mod support;
pub (crate) use support::*;
pub fn init(config: Option<temper_core::Config>) -> temper_core::Result<temper_core::AsyncRunner> {
    crate::CONFIG.get_or_init(| | config.unwrap_or_else(| | temper_core::Config::default()));
    temper_std::init(Some(crate::config().clone())) ? ;
    src::init() ? ;
    Ok(crate::config().runner().clone())
}
