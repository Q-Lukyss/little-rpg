use super::types::{Entity, Stats};
use crate::inventory::{weapon::OffHand};

pub trait HasStats {
    fn stats(&self) -> &Stats;
    fn stats_mut(&mut self) -> &mut Stats;
}
impl HasStats for Entity {
    fn stats(&self) -> &Stats { &self.stats }
    fn stats_mut(&mut self) -> &mut Stats { &mut self.stats }
}

pub trait AttackProfile {
    fn attack_value(&self) -> i32;
    fn defense_value(&self) -> i32;
}

impl AttackProfile for Entity {
    fn attack_value(&self) -> i32 {
        let base = self.stats.attack;
        let main = &self.equipment.main;
        let off_bonus = match self.equipment.off.as_ref() {
            Some(OffHand::Weapon(w)) => w.base_damage as i32,
            _ => 0,
        };
        base + main.base_damage as i32 + off_bonus
    }

    fn defense_value(&self) -> i32 {
        let base = self.stats.defense;
        let main = &self.equipment.main;
        let off_block = match self.equipment.off.as_ref() {
            Some(OffHand::Shield(s)) => s.dmg_reduction as i32,
            Some(OffHand::Weapon(w)) => w.dmg_reduction_parry as i32,
            None => 0,
        };
        base + main.dmg_reduction_parry as i32 + off_block
    }
}

impl Entity {
    pub fn enforce_two_hands(&mut self) {
        use crate::inventory::weapon::Wield;
        if let Wield::TwoHands = self.equipment.main.kind.get_weapon_wielding() {
            self.equipment.off = None;
        }
    }
}
