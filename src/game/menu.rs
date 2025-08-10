use crate::state::GameState;
use super::helpers::read_line_trimmed;

pub fn run() -> GameState {
    println!("=== MENU ===");
    println!("Bienvenue dans Little RPG un jeu écrit en Rust.");
    println!("Tapez :");
    println!("1. pour afficher ce Menu");
    println!("2. pour Combattre");
    println!("3. pour Quitter");

    match read_line_trimmed().as_str() {
        "1" => GameState::Menu,
        "2" => GameState::Exploration,
        "3" => {
            println!("Fin du jeu !");
            std::process::exit(0);
        }
        _ => {
            println!("Choix invalide");
            GameState::Menu
        }
    }
}
