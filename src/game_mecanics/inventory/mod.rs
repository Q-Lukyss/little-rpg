use crate::game_mecanics::{Armor, Consumable, Map, Quest, Shield, Weapon, first_quest};

pub struct Inventory {
    pub weapons: Vec<Weapon>,
    pub armors: Vec<Armor>,
    pub shields: Vec<Shield>,
    pub consumables: Vec<Consumable>,
    pub quests: Vec<Quest>,
    pub gold: u32,
    pub maps: Vec<Map>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            weapons: Vec::new(),
            armors: Vec::new(),
            shields: Vec::new(),
            consumables: Vec::new(),
            quests: vec![first_quest()],
            gold: 0,
            maps: Vec::new(),
        }
    }
}
