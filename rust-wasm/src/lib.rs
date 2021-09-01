use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 绑定 JS 方法
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);

}

// 导出函数（可被 JS 调用）
#[wasm_bindgen]
pub fn print(s: &str) {
    log(s);
}

#[wasm_bindgen]
pub fn wasm_alert(s: &str) {
    alert(s);
}
