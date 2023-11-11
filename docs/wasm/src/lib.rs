use std::cell::RefCell;

use chain_reaction::Game;
use serde_json::to_string;
use wasm_bindgen::prelude::wasm_bindgen;

thread_local! {
static GAME: RefCell<Game> = RefCell::new(Game::default());
}

#[wasm_bindgen(js_name = "newGame")]
pub fn new_game(height: usize, width: usize, players: usize) -> bool {
    let new_game = Game::new(height, width, players);
    if let Some(game) = new_game {
        GAME.with(|g| g.replace(game));
        true
    } else {
        false
    }
}

#[wasm_bindgen(js_name = "addAtom")]
pub fn add_atom(row: usize, column: usize) -> Option<String> {
    GAME.with(|game| {
        game.borrow_mut()
            .add_atom((row, column))
            .map(|explosions| to_string(&explosions).unwrap())
    })
}

#[wasm_bindgen(js_name = "getState")]
pub fn get_state() -> String {
    GAME.with(|game| to_string(game).unwrap())
}
