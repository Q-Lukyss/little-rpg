use super::{Potion, Weapon};

#[derive(Debug, Clone)]
pub enum Item {
    Potion(Potion),
    Weapon(Weapon),
    Shield,
}

pub type Inventory = Vec<Item>;
