pub mod armor;
pub mod consumable;
pub mod shield;
pub mod weapon;

pub use armor::Armor;
pub use consumable::Consumable;
pub use shield::Shield;
pub use weapon::{Handling, Weapon};

pub enum Loot {
    Consumable(Consumable),
    Armor(Armor),
    Shield(Shield),
    Weapon(Weapon),
    Gold(u32),
    Map(Location),
}

pub enum Location {
    City(String),
    Dungeon(String),
}
