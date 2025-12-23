pub mod armor;
pub mod consumable;
pub mod map;
pub mod shield;
pub mod weapon;

pub use armor::Armor;
pub use consumable::{Consumable, Potion};
pub use map::{Location, Map};
pub use shield::Shield;
pub use weapon::{Handling, Weapon};

#[derive(Debug, Clone)]
pub enum Loot {
    Consumable(Consumable),
    Armor(Armor),
    Shield(Shield),
    Weapon(Weapon),
    Gold(u32),
    Map(Map),
}
