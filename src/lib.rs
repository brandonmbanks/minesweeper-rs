mod minesweeper;
mod random;

use minesweeper::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
    static MINESWEEPER: RefCell<Minesweeper> = RefCell::new(Minesweeper::new(10, 10, 15));
}

#[wasm_bindgen(js_name=getGame)]
pub fn get_game() -> String {
    MINESWEEPER.with(|ms| {
        format!("{{\"game\": \"{}\", \"gameState\": \"{}\"}}", ms.borrow().to_string(), ms.borrow().get_state())
    })
}

#[wasm_bindgen(js_name=revealCell)]
pub fn reveal_cell(x: usize, y: usize) {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().reveal((x, y));
    });
}

#[wasm_bindgen(js_name=toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().toggle_flag((x, y));
    });
}
