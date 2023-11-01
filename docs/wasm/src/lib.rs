use std::cell::RefCell;

use chain_reaction::Game;
use serde_json::{to_string, Number, Value};
use wasm_bindgen::prelude::wasm_bindgen;

thread_local! {
static GAME: RefCell<Game> = RefCell::new(Game::new(0, 0, 0));
}

// #[wasm_bindgen(js_name = "newGame")]
pub fn new_game(height: usize, width: usize, max_history: usize, seed: String) -> String {
    unimplemented!()
}

// #[wasm_bindgen(js_name = "addAtom")]
pub fn add_atom(row: usize, column: usize) -> Option<String> {
    unimplemented!();
}

// #[wasm_bindgen(js_name = "getState")]
pub fn get_state() -> String {
    unimplemented!();
}
