pub mod menu;
pub mod exploration;
pub mod combat;
pub mod inventory;
pub mod helpers;

use crate::entity::Entity;
use crate::state::GameState;

pub fn run_game_loop(mut player: Entity) {
    let mut state = GameState::Menu;

    loop {
        state = match state {
            GameState::Menu => menu::run(),
            GameState::Exploration => exploration::run(&mut player),
            GameState::Combat(mut enemies) => combat::run(&mut player, &mut enemies),
            GameState::Inventory => inventory::run(&mut player),
            GameState::GameOver => {
                println!("Tu es mort. Fin du jeu.");
                break;
            }
        };
    }
}
