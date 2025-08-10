use crate::player::Player;
use crate::state::GameState;
use crate::inventory::Item;
use super::helpers::read_line_trimmed;

pub fn run(player: &mut Player) -> GameState {
    println!("=== Gestion de l'inventaire ===");
    println!("1. Voir les armes");
    println!("2. Retour");

    match read_line_trimmed().as_str() {
        "1" => {
            let weapons: Vec<(usize, &Item)> = player
                .inventory
                .iter()
                .enumerate()
                .filter(|(_, item)| matches!(item, Item::Weapon(_)))
                .collect();

            if weapons.is_empty() {
                println!("Aucune arme dans l'inventaire.");
            } else {
                println!("=== Armes ===");
                for (i, item) in &weapons {
                    if let Item::Weapon(w) = item {
                        println!("{}. {:?} - dmg: {}", i + 1, w.kind, w.base_damage);
                    }
                }

                println!("Quelle arme veux-tu équiper ?");
                if let Ok(index) = read_line_trimmed().parse::<usize>() {
                    if let Some((real_index, Item::Weapon(w))) =
                        weapons.into_iter().find(|(i, _)| *i + 1 == index)
                    {
                        player.equip_weapon(w.clone());
                        player.inventory.remove(real_index);
                    }
                }
            }
            GameState::Inventory
        }
        "2" => GameState::Menu,
        _ => {
            println!("Choix invalide.");
            GameState::Inventory
        }
    }
}
