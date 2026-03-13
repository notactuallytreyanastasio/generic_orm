#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
pub mod json;
pub mod temporal;
pub mod testing;
pub mod net;
pub mod regex;
mod support;
pub (crate) use support::*;
pub fn init(config: Option<temper_core::Config>) -> temper_core::Result<temper_core::AsyncRunner> {
    crate::CONFIG.get_or_init(| | config.unwrap_or_else(| | temper_core::Config::default()));
    json::init() ? ;
    temporal::init() ? ;
    testing::init() ? ;
    net::init() ? ;
    regex::init() ? ;
    Ok(crate::config().runner().clone())
}
