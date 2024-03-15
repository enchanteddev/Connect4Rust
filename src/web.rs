mod main;

use std::f64::INFINITY;

use wasm_bindgen::prelude::*;
use main::get_max_move;

#[wasm_bindgen]
pub fn get_move(board: Vec<u8>, level: i32, player: u8) -> i8 {
    let mut nboard: Vec<Vec<u8>> = vec![vec![0; 7]; 6];
    for (i, elem) in board.iter().enumerate() {
        let rowi = i / 7;
        let coli = i % 7;
        nboard[rowi][coli] = *elem;
    }
    get_max_move(&nboard, level, player, INFINITY).1
}