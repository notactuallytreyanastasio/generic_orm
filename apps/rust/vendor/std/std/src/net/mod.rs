#![allow(warnings)]
#![allow(dependency_on_unit_never_type_fallback)]
use temper_core::AnyValueTrait;
use temper_core::AsAnyValue;
use temper_core::Pair;
mod support;
pub use support::*;
pub (crate) fn init() -> temper_core::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<temper_core::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE.get_or_init(| |{
            Ok(())
    }).clone()
}
struct NetRequestStruct {
    url: std::sync::Arc<String>, method: std::sync::Arc<String>, body_content: Option<std::sync::Arc<String>>, body_mime_type: Option<std::sync::Arc<String>>
}
#[derive(Clone)]
pub struct NetRequest(std::sync::Arc<std::sync::RwLock<NetRequestStruct>>);
impl NetRequest {
    pub fn post(& self, content__22: impl temper_core::ToArcString, mimeType__23: impl temper_core::ToArcString) {
        let content__22 = content__22.to_arc_string();
        let mimeType__23 = mimeType__23.to_arc_string();
        self.0.write().unwrap().method = std::sync::Arc::new("POST".to_string());
        self.0.write().unwrap().body_content = Some(content__22.clone());
        let mut t___50: Option<std::sync::Arc<String>> = self.0.read().unwrap().body_mime_type.clone();
        self.0.write().unwrap().body_mime_type = t___50.clone();
    }
    pub fn send(& self) -> temper_core::Promise<NetResponse> {
        return send_request(self.0.read().unwrap().url.clone().clone(), self.0.read().unwrap().method.clone(), self.0.read().unwrap().body_content.clone(), self.0.read().unwrap().body_mime_type.clone());
    }
    pub fn new(url__28: impl temper_core::ToArcString) -> NetRequest {
        let url__28 = url__28.to_arc_string();
        let url;
        let method;
        let body_content;
        let body_mime_type;
        url = url__28.clone();
        method = std::sync::Arc::new("GET".to_string());
        body_content = None;
        body_mime_type = None;
        let selfish = NetRequest(std::sync::Arc::new(std::sync::RwLock::new(NetRequestStruct {
                        url, method, body_content, body_mime_type
        })));
        return selfish;
    }
}
temper_core::impl_any_value_trait!(NetRequest, []);
pub trait NetResponseTrait: temper_core::AsAnyValue + temper_core::AnyValueTrait + std::marker::Send + std::marker::Sync {
    fn clone_boxed(& self) -> NetResponse;
    fn status(& self) -> i32;
    fn content_type(& self) -> Option<std::sync::Arc<String>>;
    fn body_content(& self) -> temper_core::Promise<Option<std::sync::Arc<String>>>;
}
#[derive(Clone)]
pub struct NetResponse(std::sync::Arc<dyn NetResponseTrait>);
impl NetResponse {
    pub fn new(selfish: impl NetResponseTrait + 'static) -> NetResponse {
        NetResponse(std::sync::Arc::new(selfish))
    }
}
impl NetResponseTrait for NetResponse {
    fn clone_boxed(& self) -> NetResponse {
        NetResponseTrait::clone_boxed( & ( * self.0))
    }
    fn status(& self) -> i32 {
        NetResponseTrait::status( & ( * self.0))
    }
    fn content_type(& self) -> Option<std::sync::Arc<String>> {
        NetResponseTrait::content_type( & ( * self.0))
    }
    fn body_content(& self) -> temper_core::Promise<Option<std::sync::Arc<String>>> {
        NetResponseTrait::body_content( & ( * self.0))
    }
}
temper_core::impl_any_value_trait_for_interface!(NetResponse);
impl std::ops::Deref for NetResponse {
    type Target = dyn NetResponseTrait;
    fn deref(& self) -> & Self::Target {
        & ( * self.0)
    }
}
fn sendRequest__16(url__35: impl temper_core::ToArcString, method__36: impl temper_core::ToArcString, bodyContent__37: Option<impl temper_core::ToArcString>, bodyMimeType__38: Option<impl temper_core::ToArcString>) -> temper_core::Promise<NetResponse> {
    let url__35 = url__35.to_arc_string();
    let method__36 = method__36.to_arc_string();
    let bodyContent__37 = bodyContent__37.map(| x | x.to_arc_string());
    let bodyMimeType__38 = bodyMimeType__38.map(| x | x.to_arc_string());
    return panic!();
}
