// Déclare les sous-modules situés dans src/player/
pub mod base;
pub mod factory;
pub mod combat;
pub mod xp;
pub mod buffs;
pub mod stats;

// Réexporte pour ne rien casser ailleurs
pub use base::{Player, ActiveBuff};
pub use stats::Stat;
