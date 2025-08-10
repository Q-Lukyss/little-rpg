pub mod base;
pub mod potion;
pub mod stats;
pub mod weapon;

// Réexports pour garder la même API publique
pub use base::{Item, Inventory};
pub use potion::Potion;
pub use stats::Stat;
pub use weapon::{WeaponKind, AttackPattern, Weapon};
