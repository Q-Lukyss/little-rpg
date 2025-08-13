// Déclare les sous-modules (fichiers attendus : src/entity/types.rs, traits.rs, combat.rs, player.rs, enemy.rs, powers.rs)
pub mod types;
pub mod traits;
pub mod combat;
pub mod player;
pub mod enemy;
pub mod powers;

// Ré-exports pour offrir une API propre à l'extérieur du module
pub use types::{Entity, Stats, Equipment, Kind, PlayerData, EnemyData, ActiveBuff, EnemyType, EnemyRank};
pub use traits::{HasStats, AttackProfile};
pub use player::PlayerView;
pub use combat::resolve_attack;
pub use player::make_player;
pub use enemy::{
    spawn, spawn_multiple, spawn_boss, spawn_with_type_and_rank,
    EnemyView, DEFAULT_RATES, roll_rank,
};
pub use powers::InnatePower;
