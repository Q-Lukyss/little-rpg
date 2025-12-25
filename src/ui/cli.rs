use crate::core::event::CombatEvent;
use crate::core::{
    Action, CombatAction, Event, ExplorationAction, ExplorationEvent, Game, GameState,
};
use crate::game_mecanics::{Consumable, Loot};
use colored::Colorize;
use std::io::{self, Write};

pub struct Cli;

impl Cli {
    pub fn render(&mut self, game: &Game) {
        match game.state {
            GameState::Exploration => {
                println!("\n{}", "=== Exploration ===".green());
                println!(
                    "{} | PV: {}/{}.",
                    game.player.name, game.player.stats.hp, game.player.stats.max_hp,
                );
                println!("Que veux-tu faire ?");
                println!("1. Explorer");
                println!("2. Inventaire");
                println!("3. Menu");
                println!("q. Quitter");
            }
            GameState::Combat => {
                println!("\n{}", "=== Combat ===".red());
                println!(
                    "{} | PV: {}/{}.",
                    game.player.name, game.player.stats.hp, game.player.stats.max_hp
                );
                println!("Que veux-tu faire ?");
                println!("1. Attaquer");
                println!("2. Parrer");
                println!("3. Bloquer");
                println!("i. Inventaire");
                println!("f. Fuir");
                println!("q. Quitter");
            }
            GameState::Inventory => {
                println!("\n{}", "=== Inventory ===".yellow());
                println!(
                    "{} | PV: {}/{}.",
                    game.player.name, game.player.stats.hp, game.player.stats.max_hp
                );
                println!("(Inventory pas encore implémenté dans cet exemple)");
                println!("q. Quitter");
            }
            GameState::Menu => {
                println!("\n{}", "=== Menu ===".yellow());
                println!("(Menu pas encore implémenté dans cet exemple)");
                println!("q. Quitter");
            }
            GameState::GameOver => {
                println!("\n{}", "=== Game Over ===".red());
                println!("Tu es mort.");
            }
        }
    }

    pub fn render_events(&mut self, events: &[Event]) {
        for e in events {
            match e {
                Event::Text(t) => println!("{t}"),
                Event::FindLoot(Loot::Consumable(Consumable::Potion(potion))) => {
                    println!("Tu trouves une potion : {} ", potion.name);
                }
                Event::FindLoot(Loot::Shield(shield)) => {
                    println!("Tu trouves un bouclier : {} ", shield.name);
                }
                Event::FindLoot(Loot::Weapon(weapon)) => {
                    println!("Tu trouves une arme : {} ", weapon.name);
                }
                Event::FindLoot(Loot::Armor(armor)) => {
                    println!("Tu trouves une armure : {} ", armor.name);
                }
                Event::Exploration(ExplorationEvent::EncounterEnemy(enemy_vec)) => {
                    match enemy_vec.len() {
                        1 => println!("Un ennemi apparaît !"),
                        _ => println!("{} ennemis apparaissent", enemy_vec.len()),
                    }
                    for enemy in enemy_vec {
                        let name = enemy.name.clone().unwrap_or_else(|| "Gobelin".to_string());
                        println!("{} : {}/{}hp", name, enemy.stats.hp, enemy.stats.max_hp);
                    }
                }
                Event::EnterCombat => {
                    println!("un Combat Commence !");
                }
                Event::Quit => {
                    println!("Vous quittez le jeu.");
                }
                //Combat events
                Event::CombatEvent(CombatEvent::Attack) => {
                    println!("Vous attaquez le(s) ennemi(s) !");
                }
                Event::CombatEvent(CombatEvent::EnemyDefeated(enemy)) => {
                    print!(
                        "Vous avez vaincu {}.",
                        enemy
                            .name
                            .clone()
                            .unwrap_or_else(|| "le Gobelin".to_string())
                    );
                }
                Event::CombatEvent(CombatEvent::Report(combat)) => {
                    println!("Combat en cours :");
                    println!(
                        "{} : {}/{}hp",
                        combat.player.name, combat.player.stats.hp, combat.player.stats.max_hp
                    );
                    for enemy in combat.opponents.clone() {
                        let name = enemy.name.clone().unwrap_or_else(|| "Gobelin".to_string());
                        println!("{} : {}/{}hp", name, enemy.stats.hp, enemy.stats.max_hp);
                    }
                }
                _ => {
                    println!("Événement nom implémenté");
                }
            }
        }
    }

    pub fn read_action(&mut self, game: &Game) -> Option<Action> {
        let input = read_line_trimmed()?;
        match game.state {
            GameState::Exploration => match input.as_str() {
                "1" => Some(Action::Exploration(ExplorationAction::Explore)),
                "2" => todo!("Gestion de l'inventaire"),
                "3" => todo!("Gestion Menu"),
                "q" | "quit" => Some(Action::Quit),
                _ => {
                    println!("Commande invalide.");
                    None
                }
            },
            GameState::Combat => match input.as_str() {
                "1" => Some(Action::Combat(CombatAction::Attack)),
                "2" => Some(Action::Combat(CombatAction::Parry)),
                "3" => Some(Action::Combat(CombatAction::Block)),
                "i" => {
                    println!("Acces inventaire pas encore implémenté");
                    None
                }
                "f" => Some(Action::Combat(CombatAction::Flee)),
                "q" | "quit" => Some(Action::Quit),
                _ => {
                    println!("Commande invalide.");
                    None
                }
            },
            GameState::Menu => match input.as_str() {
                "q" | "quit" => Some(Action::Quit),
                _ => {
                    println!("Commande invalide.");
                    None
                }
            },
            GameState::Inventory => match input.as_str() {
                "q" | "quit" => Some(Action::Quit),
                _ => {
                    println!("(Inventaire pas dispo ici) Tape q pour quitter.");
                    None
                }
            },
            GameState::GameOver => None,
        }
    }
}

fn read_line_trimmed() -> Option<String> {
    print!("> ");
    let _ = io::stdout().flush();
    let mut s = String::new();
    if io::stdin().read_line(&mut s).is_ok() {
        Some(s.trim().to_string())
    } else {
        None
    }
}
