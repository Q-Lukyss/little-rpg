use crate::entity::Entity;
use crate::inventory::{Item, Potion, AttackPattern};
use crate::state::GameState;
use super::helpers::{read_line_trimmed, can_attack_this_turn};

pub fn run(player: &mut Entity, enemies: &mut Vec<Entity>) -> GameState {
    let player_name = player.name.as_deref().unwrap_or("Héros");
    println!("{}, tu entres en combat...", player_name);

    let mut turn: u32 = 1;
    let mut next_state: Option<GameState> = None;

    let enemy = enemies.get_mut(0).expect("Combat sans ennemi");

    while enemy.stats.hp > 0 && player.stats.hp > 0 && next_state.is_none() {
        if let Some(mut pv) = player.as_player_mut() {
            pv.update_buffs();
        }

        println!("\n=== Combat (tour {turn}) ===");
        println!("Tes PV : {}/{}", player.stats.hp, player.stats.max_hp);
        println!("PV de l'ennemi : {}", enemy.stats.hp);
        println!("Que veux-tu faire ?");
        println!("1. Attaquer");
        println!("2. Défense");
        println!("3. Utiliser un objet");
        println!("4. Fuir");
        println!("5. Retour");

        match read_line_trimmed().as_str() {
            "1" => choice_attack(player, enemy, turn),
            "2" => choice_defense(player, enemy),
            "3" => choice_use_item(player),
            "4" => {
                choice_flee();
                next_state = Some(GameState::Exploration);
            }
            "5" => {
                println!("Retour à l'exploration !");
                next_state = Some(GameState::Exploration);
            }
            "demonic_eye" => {
                if let Some(pv) = player.as_player_mut() {
                    pv.demonic_eye(enemy);
                }
            }
            _ => println!("Commande invalide !"),
        };

        turn += 1;
    }

    if let Some(s) = next_state {
        s
    } else if player.stats.hp == 0 {
        GameState::GameOver
    } else {
        if let Some(ev) = enemy.as_enemy_mut() {
            let xp = ev.xp_reward();
            if let Some(mut pv) = player.as_player_mut() {
                pv.gain_xp(xp);
            }
        }
        GameState::Exploration
    }
}

fn choice_attack(player: &mut Entity, enemy: &mut Entity, turn: u32) {
    let pattern = player.equipment.main.kind.attack_pattern();

    if pattern == AttackPattern::TwicePerTurn {
        println!("Attaque 1 !");
        if let Some(pv) = player.as_player_mut() { pv.attack_entity(enemy); }
        if enemy.stats.hp > 0 {
            println!("Attaque 2 !");
            if let Some(pv) = player.as_player_mut() { pv.attack_entity(enemy); }
        }
    } else if can_attack_this_turn(&turn, &player.equipment.main.kind) {
        println!("Tu attaques !");
        if let Some(pv) = player.as_player_mut() { pv.attack_entity(enemy); }
    } else {
        println!("Ton arme ne peut pas attaquer ce tour !");
    }

    if enemy.stats.hp > 0 {
        println!("L'ennemi riposte !");
        if let Some(ev) = enemy.as_enemy_mut() {
            ev.attack_entity(player);
        }
    }
}

fn choice_use_item(player: &mut Entity) {
    // 1) Construire la liste des potions et la map d’indices (emprunt IMMUTABLE court)
    let (pot_idx_map, pot_list): (Vec<usize>, Vec<String>) = {
        let inv = match &player.kind {
            crate::entity::Kind::Player(pd) => &pd.inventory,
            _ => {
                println!("Action réservée au joueur.");
                return;
            }
        };
        let mut idxs = Vec::new();
        let mut labels = Vec::new();
        for (i, it) in inv.iter().enumerate() {
            if let Item::Potion(p) = it {
                idxs.push(i);
                labels.push(format!("{:?}", p));
            }
        }
        if idxs.is_empty() {
            println!("Tu n'as aucune potion.");
        } else {
            println!("=== Potions disponibles ===");
            for (k, label) in labels.iter().enumerate() {
                println!("{}. {}", k + 1, label);
            }
        }
        (idxs, labels)
    };

    if pot_idx_map.is_empty() {
        return;
    }

    println!("Choisis une potion (ou autre pour annuler) :");
    if let Ok(choice) = read_line_trimmed().parse::<usize>() {
        if choice == 0 || choice > pot_idx_map.len() {
            println!("Potion invalide.");
            return;
        }

        // 2) Cloner la potion choisie (emprunt IMMUTABLE très court)
        let (real_index, potion) = {
            let inv = match &player.kind {
                crate::entity::Kind::Player(pd) => &pd.inventory,
                _ => unreachable!(),
            };
            let ri = pot_idx_map[choice - 1];
            let p = match &inv[ri] {
                Item::Potion(p) => p.clone(),
                _ => {
                    println!("Sélection invalide.");
                    return;
                }
            };
            (ri, p)
        };

        // 3) Utiliser la potion (re-emprunt &mut Entity)
        if let Some(mut pv) = player.as_player_mut() {
            pv.use_potion(potion);
        }

        // 4) Retirer la potion de l’inventaire (emprunt &mut court)
        {
            let inv = match &mut player.kind {
                crate::entity::Kind::Player(pd) => &mut pd.inventory,
                _ => unreachable!(),
            };
            inv.remove(real_index);
        }
    }
}

fn choice_flee() {
    println!("Tu prends la fuite !");
}

fn choice_defense(player: &mut Entity, enemy: &mut Entity) {
    if let Some(mut pv) = player.as_player_mut() {
        pv.defend_against(enemy);
    }
}
