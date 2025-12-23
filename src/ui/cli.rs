use crate::core::{Action, Combat, Event, ExplorationAction, Game, GameState};
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
                println!("(Combat pas encore implémenté dans cet exemple)");
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
                Event::EncounterEnemy => {
                    println!("Un ennemi apparaît !");
                }
                Event::EnterCombat => {
                    println!("Vous entrez dans le combat !");
                }
                Event::Quit => {
                    println!("Vous quittez le jeu.");
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
                "q" | "quit" => Some(Action::Quit),
                _ => {
                    println!("(Combat pas dispo ici) Tape q pour quitter.");
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
