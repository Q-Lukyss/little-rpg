mod game;
mod player;
mod enemy;
mod state;
mod inventory;
mod data;

use crate::player::Player;
use crate::game::run_game_loop;

fn main() {
    let hero = Player::new("Ferris".to_string());
    run_game_loop(hero);
}
