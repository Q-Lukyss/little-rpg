use crate::inventory::{shield::Shield, Inventory, Weapon};
use super::stats::Stat;

pub struct Player {
    pub name: String,
    pub hp: (i32, i32),
    pub attack: i32,
    pub level: u32,
    pub xp: (u32, u32),
    pub inventory: Inventory,
    pub equiped_weapon: Weapon,
    pub equiped_shield : Option<Shield>,
    pub buffs: Vec<ActiveBuff>,
}

pub struct ActiveBuff {
    pub stat: Stat,
    pub value: u32,
    pub remaining_turns: u8,
}
