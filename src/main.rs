mod core;
mod game_mecanics;
mod ui;

use crate::core::{Action, Event, Game, GameState};
use crate::game_mecanics::{Exploration, Inventory, Menu};
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
            Exploration::apply(game, exploration_action)
        }
        (GameState::Menu, Action::Menu(menu_action)) => todo!(),
        (GameState::Combat, Action::Combat(combat_action)) => todo!(),
        (GameState::Inventory, Action::Inventory(inventory_action)) => todo!(),
        (GameState::GameOver, _) => {
            vec![Event::GameOver]
        }
        _ => todo!("Apply gestion autres etats main.rs"),
    }
}
