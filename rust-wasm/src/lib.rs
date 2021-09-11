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

#[wasm_bindgen]
pub fn fib(n: u32) -> u32 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

// 使用尾调用优化
#[wasm_bindgen]
pub fn fib_tail(n: u32, result: u32, total: u32) -> u32 {
    if n == 1 || n == 2 {
        return result;
    } else {
        return fib_tail(n - 1, total, result + total);
    }
}
