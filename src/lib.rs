mod random;
mod minesweeper;

use wasm_bindgen::prelude::*;

use minesweeper::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
