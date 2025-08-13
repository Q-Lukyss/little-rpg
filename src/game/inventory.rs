use crate::entity::Entity;
use crate::state::GameState;
use crate::inventory::Item;
use super::helpers::read_line_trimmed;

pub fn run(player: &mut Entity) -> GameState {
    println!("=== Gestion de l'inventaire ===");
    println!("1. Voir les armes");
    println!("2. Retour");

    match read_line_trimmed().as_str() {
        "1" => {
            // 1) Lire/afficher la liste SANS &mut, et construire une map d'indices réels
            let (indices, names_and_dmg): (Vec<usize>, Vec<(String, u32)>) = {
                let inv = match &player.kind {
                    crate::entity::Kind::Player(pd) => &pd.inventory,
                    _ => {
                        println!("Action réservée au joueur.");
                        return GameState::Menu;
                    }
                };
                let mut idxs = Vec::new();
                let mut infos = Vec::new();
                for (i, item) in inv.iter().enumerate() {
                    if let Item::Weapon(w) = item {
                        idxs.push(i);
                        infos.push((format!("{:?}", w.kind), w.base_damage));
                    }
                }
                if idxs.is_empty() {
                    println!("Aucune arme dans l'inventaire.");
                } else {
                    println!("=== Armes ===");
                    for (k, (name, dmg)) in infos.iter().enumerate() {
                        println!("{}. {} - dmg: {}", k + 1, name, dmg);
                    }
                }
                (idxs, infos)
            };

            if indices.is_empty() {
                return GameState::Inventory;
            }

            println!("Quelle arme veux-tu équiper ?");
            if let Ok(choice) = read_line_trimmed().parse::<usize>() {
                if choice == 0 || choice > indices.len() {
                    println!("Choix invalide.");
                    return GameState::Inventory;
                }

                // 2) Cloner l’arme choisie dans un petit bloc (emprunt court immuable)
                let (real_index, new_weapon) = {
                    let inv = match &player.kind {
                        crate::entity::Kind::Player(pd) => &pd.inventory,
                        _ => unreachable!(),
                    };
                    let ri = indices[choice - 1];
                    let w = match &inv[ri] {
                        Item::Weapon(w) => w.clone(),
                        _ => {
                            println!("Sélection invalide.");
                            return GameState::Inventory;
                        }
                    };
                    (ri, w)
                };

                // 3) Équiper (nouvel emprunt &mut Entity via as_player_mut)
                let old_main = player.equipment.main.clone();
                if let Some(mut pv) = player.as_player_mut() {
                    pv.equip_weapon(new_weapon);
                }

                // 4) Ré-ouvrir un emprunt &mut sur l’inventaire pour remplacer l’item
                {
                    let inv = match &mut player.kind {
                        crate::entity::Kind::Player(pd) => &mut pd.inventory,
                        _ => unreachable!(),
                    };
                    inv[real_index] = Item::Weapon(old_main);
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
