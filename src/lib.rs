mod utils;

use wasm_bindgen::prelude::*;
use uuid::Uuid;
use web_sys::window;
use js_sys::{Object,Math};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_random_number() -> String {
    // 调用JavaScript的Math.random()
    let random_num = Math::random();
    
    // 在Rust中进行处理（这里简单转换为字符串）
    let random_str = format!("Js Random Number: {:.4}", random_num);
    
    // 返回处理后的字符串
    random_str
}
/// 直接调用浏览器alert显示消息
#[wasm_bindgen]
pub fn show_alert_message(message: &str) {
    alert(message);
}

/// 生成一个新的UUID v4
#[wasm_bindgen]
pub fn generate_uuid() -> String {
    utils::set_panic_hook();
    Uuid::new_v4().to_string()
}

/// 获取浏览器环境信息
#[wasm_bindgen]
pub fn get_browser_info() -> JsValue {
    utils::set_panic_hook();
    let window = window().expect("no global window exists");
    let doc = window.document().expect("no document exists");
    let screen = window.screen().expect("no screen exists");
    let perf = window.performance().expect("performance should be available");
    let location = window.location();

    let info = Object::new();
    js_sys::Reflect::set(&info, &"url".into(), &location.href().unwrap().into()).unwrap();
    js_sys::Reflect::set(&info, &"title".into(), &doc.title().into()).unwrap();
    js_sys::Reflect::set(&info, &"screenWidth".into(), &screen.width().unwrap().into()).unwrap();
    js_sys::Reflect::set(&info, &"screenHeight".into(), &screen.height().unwrap().into()).unwrap();
    js_sys::Reflect::set(&info, &"loadTime".into(), &perf.now().into()).unwrap();
    
    info.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_uuid();
        assert!(Uuid::parse_str(&uuid).is_ok());
    }
}