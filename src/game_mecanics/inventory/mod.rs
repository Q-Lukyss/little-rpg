use crate::game_mecanics::{Armor, Consumable, Map, Quest, Shield, Trinket, Weapon, first_quest};

#[derive(Debug, Clone)]
pub struct Inventory {
    pub weapons: Vec<Weapon>,
    pub armors: Vec<Armor>,
    pub trinkets: Vec<Trinket>,
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
            trinkets: Vec::new(),
            shields: Vec::new(),
            consumables: Vec::new(),
            quests: vec![first_quest()],
            gold: 0,
            maps: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Consumable(Consumable),
    Shield(Shield),
    Trinket(Trinket),
}
