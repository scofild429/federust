mod utils;

use client_module::clientadd;
use server_module::serveradd;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let a = clientadd(30, 34).to_string();
    let b = serveradd(32, 34).to_string();
    let message = a + &b;
    alert(&message)
}
