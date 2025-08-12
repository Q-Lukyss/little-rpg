use crate::enemy::{Enemy, EnemyRank, EnemyType};
use crate::state::GameState;
use crate::player::Player;
use super::helpers::read_line_trimmed;

pub fn run(_player: &mut Player) -> GameState {
    // choisir le type
    println!("=== Phase 1 ===");
    println!("Choisir un Type d'ennemi.");
    println!("1. Gobelin");
    println!("2. Squelette");
    println!("3. Bandit");

    let enemy_type: EnemyType = loop {
        match read_line_trimmed().as_str() {
            "1" => break EnemyType::Gobelin,
            "2" => break EnemyType::Skeleton,
            "3" => break EnemyType::Human,
            _ => println!("Choix invalide, recommence."),
        }
    };

    // choisir le rang
    println!("=== Phase 2 ===");
    println!("Choisir un Rang d'ennemi.");
    println!("1. Lambda");
    println!("2. Nommé");
    println!("3. Elite");
    println!("4. Boss");
    println!("5. Légendaire");

    let enemy_rank: EnemyRank = loop {
        match read_line_trimmed().as_str() {
            "1" => break EnemyRank::Lambda,
            "2" => break EnemyRank::Named,
            "3" => break EnemyRank::Elite,
            "4" => break EnemyRank::Boss,
            "5" => break EnemyRank::Legendary,
            _ => println!("Choix invalide, recommence."),
        }
    };

    let enemy = Enemy::spawn_with_type_and_rank(enemy_type, enemy_rank);
    if let Some(name) = &enemy.name {
        println!("Un {} apparaît ! Il s'agit de {}.", enemy.enemy_type.as_str(), name );
    }
    else {
        println!("Un {} apparaît !", enemy.enemy_type.as_str());
    }

    GameState::Combat(vec![enemy])
}