use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(name)
}
// wasm-pack build --target web
#[wasm_bindgen]
extern{
    pub fn alert(name: &str);
}

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;