mod core;
mod game_mecanics;
mod ui;

use crate::core::{Action, Event, Game, GameState};
use crate::game_mecanics::{HandleCombat, HandleExploration};
use crate::ui::Cli;

fn main() {
    let mut game = Game::new("Lukyss");
    let mut ui = Cli;

    ui.render(&game);

    loop {
        let action = match ui.read_action(&game) {
            Some(a) => a,
            None => break,
        };

        let events = apply(&mut game, action);
        ui.render_events(&events);
        ui.render(&game);
        if matches!(game.state, GameState::GameOver) {
            break;
        }
    }
}

fn apply(game: &mut Game, action: Action) -> Vec<Event> {
    match (&game.state, action) {
        (GameState::Exploration, Action::Exploration(exploration_action)) => {
            HandleExploration::apply(game, exploration_action)
        }
        (GameState::Menu, Action::Menu(menu_action)) => todo!("implementer main.rs menuAction"),
        (GameState::Combat, Action::Combat(combat_action)) => {
            HandleCombat::apply(game, combat_action)
        }
        (GameState::Inventory, Action::Inventory(inventory_action)) => {
            todo!("implementer main.rs inventoryAction")
        }
        (GameState::GameOver, _) => {
            vec![Event::GameOver]
        }
        _ => todo!("Apply gestion autres etats main.rs"),
    }
}
