// Déclare les sous-modules situés dans src/player/
pub mod base;
pub mod factory;
pub mod combat;
pub mod xp;
pub mod buffs;

// Réexporte pour ne rien casser ailleurs
pub use base::{Player, ActiveBuff};
