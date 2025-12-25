use crate::core::{Event, ExplorationAction, ExplorationEvent, Game, GameState};
use crate::game_mecanics::{
    Armor, Combat, Consumable, Enemy, Location, Loot, Potion, Shield, Weapon,
};
use rand::Rng;

pub struct HandleExploration {}

impl HandleExploration {
    pub fn apply(game: &mut Game, action: ExplorationAction) -> Vec<Event> {
        match action {
            ExplorationAction::Explore => HandleExploration::explore(game),
            ExplorationAction::Travel(location) => HandleExploration::travel(game, location),
            ExplorationAction::EncounterEnemy => HandleExploration::encounter_enemy(game),
            ExplorationAction::FindLoot(loot) => HandleExploration::loot(game, loot),
        }
    }

    fn explore(game: &mut Game) -> Vec<Event> {
        let mut ev = vec![Event::Text("Tu explores les environs...".into())];

        let roll = rand::rng().random_range(0..100);

        match roll {
            0..=49 => {
                game.state = GameState::Combat;
                let enemy_vec = vec![Enemy::new_gobelin()];
                game.combat = Some(Combat::new(game.player.clone(), enemy_vec.clone())); // todo combat.new
                ev.push(Event::Exploration(ExplorationEvent::EncounterEnemy(
                    enemy_vec.clone(),
                )));
                ev.push(Event::EnterCombat);
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

    fn travel(game: &mut Game, location: Location) -> Vec<Event> {
        vec![Event::Exploration(ExplorationEvent::Travel(
            location.clone(),
        ))]
    }

    fn encounter_enemy(game: &mut Game) -> Vec<Event> {
        game.state = GameState::Combat;
        game.combat = Some(todo!(
            "Faire combat.new avec instanciation ennemi, tour etc"
        ));
        vec![Event::EnterCombat]
    }

    fn loot(game: &mut Game, loot: Loot) -> Vec<Event> {
        match loot {
            Loot::Weapon(weapon) => {
                game.player.inventory.weapons.push(weapon.clone());
                vec![Event::FindLoot(Loot::Weapon(weapon))]
            }
            Loot::Armor(armor) => {
                game.player.inventory.armors.push(armor.clone());
                vec![Event::FindLoot(Loot::Armor(armor))]
            }
            Loot::Shield(shield) => {
                game.player.inventory.shields.push(shield.clone());
                vec![Event::FindLoot(Loot::Shield(shield))]
            }
            Loot::Trinket(trinket) => {
                game.player.inventory.trinkets.push(trinket.clone());
                vec![Event::FindLoot(Loot::Trinket(trinket))]
            }
            Loot::Consumable(consumable) => {
                game.player.inventory.consumables.push(consumable.clone());
                match consumable {
                    Consumable::Potion(potion) => {
                        vec![Event::FindLoot(Loot::Consumable(Consumable::Potion(
                            potion,
                        )))]
                    }
                    Consumable::Elixir(elixir) => {
                        vec![Event::FindLoot(Loot::Consumable(Consumable::Elixir(
                            elixir,
                        )))]
                    }
                    Consumable::Key(key) => {
                        vec![Event::FindLoot(Loot::Consumable(Consumable::Key(key)))]
                    }
                    Consumable::QuestItem(quest_item) => {
                        todo!("Implementer le find de quest item")
                    }
                }
            }
            Loot::Gold(amount) => {
                game.player.inventory.gold += amount;
                vec![Event::FindLoot(Loot::Gold(amount))]
            }
            Loot::Map(map) => {
                game.player.inventory.maps.push(map.clone());
                vec![Event::FindLoot(Loot::Map(map))]
            }
        }
    }
}
