mod game;
mod state;
mod inventory;
mod data;
mod entity;

use crate::game::run_game_loop;
use crate::entity::make_player;

fn main() {
    let hero = make_player("Ferris".to_string());
    run_game_loop(hero);
}
