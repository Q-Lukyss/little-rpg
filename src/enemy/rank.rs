use serde::{Serialize, Deserialize};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum EnemyRank { 
    Lambda, 
    Named, 
    Elite, 
    Boss, 
    Legendary 
}

impl EnemyRank {
    pub fn stat_multiplier(self) -> f32 {
        match self {
            EnemyRank::Lambda => 1.0,
            EnemyRank::Named => 1.5,
            EnemyRank::Elite => 5.0,
            EnemyRank::Boss => 3.0,
            EnemyRank::Legendary => 10.0,
        }
    }
    pub fn xp_reward(self) -> u32 {
        match self {
            EnemyRank::Lambda => 20,
            EnemyRank::Named => 50,
            EnemyRank::Elite => 100,
            EnemyRank::Boss => 200,
            EnemyRank::Legendary => 500,
        }
    }
}
