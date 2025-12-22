use crate::game_mecanics::{Armor, Consumable, Quest, Shield, Weapon, first_quest};

pub struct Inventory {
    pub weapons: Vec<Weapon>,
    pub armors: Vec<Armor>,
    pub shields: Vec<Shield>,
    pub consumables: Vec<Consumable>,
    pub quests: Vec<Quest>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            weapons: Vec::new(),
            armors: Vec::new(),
            shields: Vec::new(),
            consumables: Vec::new(),
            quests: vec![first_quest()],
        }
    }
}
