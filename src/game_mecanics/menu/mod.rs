use crate::core::{Action, CombatState, Event, Game, GameState};
use crate::game_mecanics::{Armor, Consumable, Loot, Potion, Shield, Weapon};

pub struct Menu {}

impl Menu {
    pub fn apply(game: &mut Game, action: Action) -> Vec<Event> {
        let ev = vec![Event::Text("Menu".into())];

        ev
    }
}
