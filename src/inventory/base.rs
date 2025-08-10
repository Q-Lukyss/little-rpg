use super::{Potion, Weapon};

#[derive(Debug)]
pub enum Item {
    Potion(Potion),
    Weapon(Weapon),
    Shield,
}

pub type Inventory = Vec<Item>;
