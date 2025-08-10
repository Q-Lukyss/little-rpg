// src/enemy.rs
pub mod rank;
pub mod kind;
pub mod base;
pub mod spawn_rate;
pub mod factory;

// Ré-export pour ne rien casser ailleurs
pub use rank::EnemyRank;
pub use kind::EnemyType;
pub use base::Enemy;
