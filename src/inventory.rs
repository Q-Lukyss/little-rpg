pub mod base;
pub mod potion;
pub mod weapon;
pub mod shield;

// Réexports pour garder la même API publique
pub use base::{Item, Inventory};
pub use potion::Potion;
pub use weapon::{WeaponKind, AttackPattern, Weapon};
