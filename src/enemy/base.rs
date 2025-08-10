use super::{EnemyRank, EnemyType};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: Option<String>,
    pub hp: i32,
    pub attack: i32,
    pub enemy_type: EnemyType,
    pub rank: EnemyRank,
}
