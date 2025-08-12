use crate::enemy::Enemy;
use crate::game::combat;
use crate::player::{self, Player};
use crate::state::GameState;
use crate::inventory::{Item, Potion, AttackPattern};
use super::helpers::{read_line_trimmed, can_attack_this_turn};

pub fn run(player: &mut Player, enemies: &mut Vec<Enemy>) -> GameState {
    println!("{}, tu entre en combat...", player.name);

    let mut turn = 1;
    let mut next_state: Option<GameState> = None;
    let mut enemy = &mut enemies[0]; // il n'y a qu'un ennemi pour l'instant

    while enemy.hp > 0 && player.hp.0 > 0 && next_state.is_none() {
        player.update_buffs();

        println!("\n=== Combat ===");
        println!("Tes PV : {}/{}", player.hp.0, player.hp.1);
        println!("PV de l'ennemi : {}", enemy.hp);
        println!("Que veux-tu faire ?");
        println!("1. Attaquer");
        if player.equiped_shield.is_some() {
            println!("2. Bloquer avec le bouclier)");
        }
        else {
            println!("2. Parer avec l'arme");
        }
        println!("3. Utiliser un objet");
        println!("4. Fuir");
        println!("5. Retour");

        match read_line_trimmed().as_str() {
            "1" => {
                choice_attack(player, enemy, &turn);
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
            "4" => {
                println!("Retour à l'exploration !");
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

enum CombatAction {
    Attack,
    UseItem,
    Flee,
    Defense,
}


pub fn choice_attack(player: &mut Player, enemy: &mut Enemy, turn : &u32) {
    let pattern = player.equiped_weapon.kind.attack_pattern();
 
    if pattern == AttackPattern::TwicePerTurn {
        println!("Attaque 1 !");
        player.attack(enemy);
        if enemy.hp > 0 {
            println!("Attaque 2 !");
            player.attack(enemy);
        }
    } else if can_attack_this_turn(& turn, &player.equiped_weapon.kind) {
        println!("Tu attaques !");
        player.attack(enemy);
    } else {
        println!("Ton arme ne peut pas attaquer ce tour !");
    }

    if enemy.hp > 0 {
        println!("L'ennemi riposte !");
        enemy.attack(player);
    }
}
fn choice_use_item(){
    unimplemented!()
}
fn choice_flee(){
    // drop les items gagnés jusqu ici dans le donjons
    // retour a l'exploration
    unimplemented!()
}
fn choice_defense(){
    unimplemented!()
}



// Utilisateur peut Attaquer 
// Utilisateur s'il a un bouclier equipé peut bloquer
// utilisateur s'il a une arme peut parrer
// utilisateur peut utiliser une potion de heal
// utilisateur peut utiliser une potion de buff
// utilisateur peut fuir