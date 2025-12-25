use crate::core::event::CombatEvent;
use crate::core::{CombatAction, Event, Game, GameState};
use crate::game_mecanics::entity::{MainHand, OffHand, OffHandState};
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

    fn resolve_turn(&mut self, player_action: CombatAction, enemy_action: CombatAction) {
        match (player_action, enemy_action) {
            (CombatAction::Attack, CombatAction::Attack) => {
                todo!("player_action : attack, enemy_action : attack")
            }
            (CombatAction::Attack, CombatAction::Parry) => {
                todo!("player_action : attack, enemy_action : parry")
            }
            (CombatAction::Attack, CombatAction::Block) => {
                todo!("player_action : attack, enemy_action : block")
            }
            (CombatAction::Parry, CombatAction::Attack) => {
                todo!("player_action : parry, enemy_action : attack")
            }
            (CombatAction::Parry, CombatAction::Parry) => {
                todo!("player_action : parry, enemy_action : parry")
            }
            (CombatAction::Parry, CombatAction::Block) => {
                todo!("player_action : parry, enemy_action : block")
            }
            (CombatAction::Block, CombatAction::Attack) => {
                todo!("player_action : block, enemy_action : attack")
            }
            (CombatAction::Block, CombatAction::Parry) => {
                todo!("player_action : block, enemy_action : parry")
            }
            (CombatAction::Block, CombatAction::Block) => {
                todo!("player_action : block, enemy_action : block")
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
    pub fn apply(game: &mut Game, action: CombatAction) -> Vec<Event> {
        let combat = game
            .combat
            .as_mut()
            .expect("HandleCombat appelé avec combat = None");

        let state = mem::replace(&mut combat.state, CombatState::PlayerAction);
        match (state, action) {
            (CombatState::PlayerAction, combat_action) => {
                let mut ev: Vec<Event> = vec![];
                match combat_action {
                    CombatAction::Attack => {
                        combat.resolve_player_attack();
                        ev.push(Event::CombatEvent(CombatEvent::Attack));
                        ev.push(Event::CombatEvent(CombatEvent::Report(combat.clone())));
                        for enemy in combat.opponents.iter().filter(|e| e.stats.hp <= 0) {
                            ev.push(Event::CombatEvent(CombatEvent::EnemyDefeated(
                                enemy.clone(),
                            )));
                        }
                        combat.opponents.retain(|e| !e.is_dead());
                        if combat.opponents.is_empty() {
                            game.state = GameState::Exploration;
                        }
                    }
                    CombatAction::Parry => ev.push(Event::CombatEvent(CombatEvent::Parry)),
                    CombatAction::Block => ev.push(Event::CombatEvent(CombatEvent::Block)),
                    CombatAction::Flee => ev.push(Event::CombatEvent(CombatEvent::Flee)),
                    CombatAction::UseItem(item) => {
                        ev.push(Event::CombatEvent(CombatEvent::UseItem(item)));
                    }
                }

                ev
            }
            (CombatState::Resolve(action), _) => {
                todo!()
            }
            (CombatState::End, _) => {
                todo!()
            }
        }
    }
}
