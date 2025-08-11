use super::base::{Player, ActiveBuff};
use crate::inventory::{shield::Shield, Item, Potion, Weapon, WeaponKind};

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            hp: (20, 20),
            attack: 0,
            level: 1,
            xp: (0, 100),
            inventory: vec![Item::Potion(Potion::Healing(10))],
            equiped_weapon: Weapon { kind: WeaponKind::Sword, base_damage: 5 },
            equiped_shield: Some(Shield { armor : 5 }),
            buffs: vec![],
        }
    }
}
