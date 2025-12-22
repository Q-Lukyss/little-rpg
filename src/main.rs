mod core;
mod game_mecanics;
mod ui;

use crate::core::{Action, CombatState, Enemy, Event, Game, GameState};
use crate::game_mecanics::loot::consumable::Potion;
use crate::game_mecanics::{Armor, Consumable, Inventory, Loot, Shield, Stat, Weapon};
use crate::ui::Cli;
use rand::Rng;

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
        (GameState::Exploration, _) => explore(game),
        (_, Action::Quit) => vec![Event::Text("À bientôt.".into())],
        (_, other) => vec![Event::Text(format!("Action impossible ici: {other:?}"))],
    }
}

fn explore(game: &mut Game) -> Vec<Event> {
    let mut ev = vec![Event::Text("Tu explores les environs...".into())];

    let roll = rand::rng().random_range(0..100);

    match roll {
        0..=49 => {
            // let enemy = Enemy {
            //     name: None,
            //     stats: Stat::new(10, 10, 10, 2),
            //     inventory: Inventory::new(),
            // };
            ev.push(Event::EncounterEnemy);

            game.state = GameState::Combat;
            game.combat = Some(CombatState::Start);
            ev.push(Event::EnterCombat);
            ev.push(Event::Text("Tu entres en combat !".into()));
        }
        50..=74 => {
            let new_potion = Potion::get_basic_potion();
            game.player
                .inventory
                .consumables
                .push(Consumable::Potion(new_potion.clone()));
            ev.push(Event::FindLoot(Loot::Consumable(Consumable::Potion(
                new_potion.clone(),
            ))));
        }
        75..=80 => {
            let loot_roll = rand::rng().random_range(0..=2);
            match loot_roll {
                0 => {
                    let new_shield = Shield::get_first_shield();
                    game.player.inventory.shields.push(new_shield.clone());
                    ev.push(Event::FindLoot(Loot::Shield(new_shield.clone())));
                }
                1 => {
                    let new_weapon = Weapon::get_first_weapon();
                    game.player.inventory.weapons.push(new_weapon.clone());
                    ev.push(Event::FindLoot(Loot::Weapon(new_weapon.clone())));
                }
                2 => {
                    let new_armor = Armor::get_first_armor();
                    game.player.inventory.armors.push(new_armor.clone());
                    ev.push(Event::FindLoot(Loot::Armor(new_armor.clone())));
                }
                _ => unreachable!(),
            }
        }
        _ => {
            ev.push(Event::Text("Rien d'intéressant cette fois.".into()));
        }
    }

    ev
}
