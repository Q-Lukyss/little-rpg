use crate::core::event::CombatEvent;
use crate::core::{App, CombatAction, Event, GameState};
use crate::game_mecanics::entity::{Alive, MainHand, OffHand, OffHandState};
use crate::game_mecanics::{Armor, Enemy, Entity, Player, Shield, Trinket, Weapon, combat};
use rand::Rng;
use std::mem;

#[derive(Debug, Clone)]
pub enum CombatState {
    PlayerAction,
    Resolve(CombatAction),
    End,
}

#[derive(Debug, Clone)]
pub struct Combat {
    pub player: Player,
    pub opponents: Vec<Enemy>,
    pub state: CombatState,
    pub turn: u32,
}

impl Combat {
    pub fn new(player: Player, opponents: Vec<Enemy>) -> Self {
        Self {
            player,
            opponents,
            state: CombatState::PlayerAction,
            turn: 0,
        }
    }

    fn resolve_player_attack(&mut self) {
        let weapon_dmg = match self.player.equipment.main_hand.clone() {
            Some(MainHand::Weapon(weapon)) => weapon.damage,
            None => 0,
        };

        let attacker_total_dmg = weapon_dmg + self.player.stats.base_attack;

        for defender in self.opponents.iter_mut() {
            let shield_def = match defender.equipment.off_hand.clone() {
                OffHandState::Equipped(OffHand::Shield(shield)) => shield.defense,
                _ => 0,
            };

            let armor_def = match defender.equipment.armor.clone() {
                Some(armor) => armor.defense,
                None => 0,
            };

            let defender_total_defense = shield_def + armor_def + defender.stats.base_defense;

            if attacker_total_dmg > defender_total_defense {
                if defender.stats.hp < attacker_total_dmg + defender_total_defense {
                    defender.stats.hp = 0;
                } else {
                    defender.stats.hp -= attacker_total_dmg - defender_total_defense;
                }
            }
        }

        self.turn += 1;
    }

    fn resolve_turn(
        &mut self,
        player_action: CombatAction,
        enemy_action: CombatAction,
        enemy: &mut Enemy,
    ) {
        match (player_action, enemy_action) {
            (CombatAction::Attack, CombatAction::Attack) => {
                println!("player_action : attack, enemy_action : attack");
                let player_total_dmg = self.player.stats.base_attack
                    + match self.player.equipment.main_hand.clone() {
                        Some(MainHand::Weapon(weapon)) => weapon.damage,
                        None => 0,
                    }
                    + match self.player.equipment.off_hand.clone() {
                        OffHandState::Equipped(OffHand::Weapon(weapon)) => weapon.damage,
                        _ => 0,
                    };
                let enemy_total_dmg = enemy.stats.base_attack
                    + match enemy.equipment.main_hand.clone() {
                        Some(MainHand::Weapon(weapon)) => weapon.damage,
                        None => 0,
                    }
                    + match enemy.equipment.off_hand.clone() {
                        OffHandState::Equipped(OffHand::Weapon(weapon)) => weapon.damage,
                        _ => 0,
                    };

                match player_total_dmg > enemy_total_dmg {
                    true => enemy.take_damage(player_total_dmg - enemy_total_dmg),
                    false => self.player.take_damage(enemy_total_dmg - player_total_dmg),
                }
            }
            (CombatAction::Attack, CombatAction::Parry) => {
                println!("player_action : attack, enemy_action : parry")
            }
            (CombatAction::Attack, CombatAction::Block) => {
                println!("player_action : attack, enemy_action : block")
            }
            (CombatAction::Parry, CombatAction::Attack) => {
                println!("player_action : parry, enemy_action : attack")
            }
            (CombatAction::Parry, CombatAction::Parry) => {
                println!("player_action : parry, enemy_action : parry")
            }
            (CombatAction::Parry, CombatAction::Block) => {
                println!("player_action : parry, enemy_action : block")
            }
            (CombatAction::Block, CombatAction::Attack) => {
                println!("player_action : block, enemy_action : attack")
            }
            (CombatAction::Block, CombatAction::Parry) => {
                println!("player_action : block, enemy_action : parry")
            }
            (CombatAction::Block, CombatAction::Block) => {
                println!("player_action : block, enemy_action : block")
            }
            (_, _) => {
                todo!("Pattern autre que combinaison Attack, Parry, Block")
            }
        }
        self.turn += 1;
    }
}

pub struct HandleCombat {}

impl HandleCombat {
    pub fn apply(app: &mut App, action: CombatAction) {}
}
