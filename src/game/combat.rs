use crate::enemy::Enemy;
use crate::player::Player;
use crate::state::GameState;
use crate::inventory::{Item, Potion, AttackPattern};
use super::helpers::{read_line_trimmed, can_attack_this_turn};

pub fn run(player: &mut Player, mut enemy: Enemy) -> GameState {
    println!("{}, tu entre en combat...", player.name);

    let mut turn = 1;
    let mut next_state: Option<GameState> = None;

    while enemy.hp > 0 && player.hp.0 > 0 && next_state.is_none() {
        player.update_buffs();

        println!("\n=== Combat ===");
        println!("Tes PV : {}/{}", player.hp.0, player.hp.1);
        println!("PV de l'ennemi : {}", enemy.hp);
        println!("Que veux-tu faire ?");
        println!("1. Attaquer");
        println!("2. Utiliser un objet");
        println!("3. Fuir");

        match read_line_trimmed().as_str() {
            "1" => {
                let pattern = player.equiped_weapon.kind.attack_pattern();

                if pattern == AttackPattern::TwicePerTurn {
                    println!("Attaque 1 !");
                    player.attack(&mut enemy);
                    if enemy.hp > 0 {
                        println!("Attaque 2 !");
                        player.attack(&mut enemy);
                    }
                } else if can_attack_this_turn(turn, &player.equiped_weapon.kind) {
                    println!("Tu attaques !");
                    player.attack(&mut enemy);
                } else {
                    println!("Ton arme ne peut pas attaquer ce tour !");
                }

                if enemy.hp > 0 {
                    println!("L'ennemi riposte !");
                    player.take_dmg(enemy.attack);
                }
            }
            "2" => {
                let potions: Vec<(usize, &Item)> = player
                    .inventory
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| matches!(item, Item::Potion(_)))
                    .collect();

                if potions.is_empty() {
                    println!("Tu n'as aucune potion.");
                } else {
                    println!("=== Potions disponibles ===");
                    for (i, item) in &potions {
                        println!("{}. {:?}", i + 1, item);
                    }

                    println!("Choisis une potion (ou autre pour annuler) :");
                    if let Ok(index) = read_line_trimmed().parse::<usize>() {
                        if let Some((real_index, Item::Potion(potion))) =
                            potions.into_iter().find(|(i, _)| *i + 1 == index)
                        {
                            player.use_potion(potion.clone());
                            player.inventory.remove(real_index);
                        } else {
                            println!("Potion invalide.");
                        }
                    }
                }
            }
            "3" => {
                println!("Tu prends la fuite !");
                next_state = Some(GameState::Exploration);
            },
            "demonic_eye" => {
                player.demonic_eye(&mut enemy)
            }
            _ => {
                println!("Commande invalide !");
            }
        };

        turn += 1;
    }

    if let Some(s) = next_state {
        s
    } else if player.hp.0 <= 0 {
        GameState::GameOver
    } else {
        player.gain_xp(enemy.rank.xp_reward());
        GameState::Exploration
    }
}
