use std::cell::RefCell;

use chain_reaction::Game;
use serde_json::to_string;
use wasm_bindgen::prelude::wasm_bindgen;

thread_local! {
static GAME: RefCell<Game> = RefCell::new(Game::default());
}

#[wasm_bindgen(js_name = "newGame")]
pub fn new_game(large: bool, players: usize) {
    let new_game = if large {
        Game::large(players)
    } else {
        Game::small(players)
    };
    GAME.with(|g| g.replace(new_game));
}

#[wasm_bindgen(js_name = "addAtom")]
pub fn add_atom(row: usize, column: usize) -> String {
    GAME.with(|game| to_string(&game.borrow_mut().add_atom((row, column)).ok()))
        .unwrap()
}

#[wasm_bindgen(js_name = "getState")]
pub fn get_state() -> String {
    GAME.with(|game| to_string(game).unwrap())
}

#[wasm_bindgen]
pub fn undo() -> String {
    GAME.with(|game| game.borrow_mut().undo());
    get_state()
}
