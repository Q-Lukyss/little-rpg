use crate::game_mecanics::{Armor, Shield, Weapon};

#[derive(Debug, Clone)]
pub enum Consumable {
    Potion(Potion),
    Elixir(ElixirType),
    Key(Key),
    QuestItem(QuestItem),
}
#[derive(Debug, Clone)]
pub enum ElixirType {
    Damage,
    Health,
    Defense,
    Speed,
}

#[derive(Debug, Clone)]
pub struct Key {
    pub name: String,
    pub description: String,
    pub location_id: u32,
}

#[derive(Debug, Clone)]
pub enum QuestItem {
    Weapon(Weapon),
    Armor(Armor),
    Shield(Shield),
    Key(Key),
}

#[derive(Debug, Clone)]
pub struct Potion {
    pub name: String,
    pub description: String,
    pub heal_amount: u32,
}

impl Potion {
    pub fn new(name: String, description: String, heal_amount: u32) -> Self {
        Potion {
            name,
            description,
            heal_amount,
        }
    }
    pub fn get_basic_potion() -> Potion {
        Potion::new(
            "Potion basique".into(),
            "Une potion de base qui rend 25 points de vie".into(),
            25,
        )
    }
}
