pub mod menu;
pub mod exploration;
pub mod combat;
pub mod inventory;
pub mod helpers;

use crate::player::Player;
use crate::state::GameState;

pub fn run_game_loop(mut player: Player) {
    let mut state = GameState::Menu;

    loop {
        state = match state {
            GameState::Menu => menu::run(),
            GameState::Exploration => exploration::run(&mut player),
            GameState::Combat(enemy) => combat::run(&mut player,  enemy),
            GameState::Inventory => inventory::run(&mut player),
            GameState::GameOver => {
                println!("Tu es mort. Fin du jeu.");
                break;
            }
        };
    }
}

