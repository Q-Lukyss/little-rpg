use rand::Rng;
use crate::data::enemies::pick_name_from_db;

use super::{Enemy, EnemyType, EnemyRank};
use super::spawn_rate::{DEFAULT_RATES, roll_rank};

#[derive(Debug, Clone, Copy)]
struct BaseStats { 
    hp: i32, 
    atk: i32 
}

impl BaseStats {
    fn scaled(self, rank: EnemyRank) -> (i32, i32) {
        let m = rank.stat_multiplier();
        (((self.hp as f32) * m) as i32, ((self.atk as f32) * m) as i32)
    }
}

impl Enemy {
    pub fn spawn(enemy_type: EnemyType) -> Self {
        let mut rng = rand::rng();
        let rank = roll_rank(&mut rng, DEFAULT_RATES);
        Self::spawn_with_type_and_rank(enemy_type, rank)
    }

    pub fn spawn_multiple(enemy_type: EnemyType, count: usize) -> Vec<Self> {
        (0..count).map(|_| Self::spawn(enemy_type)).collect()
    }

    pub fn spawn_boss(enemy_type: EnemyType) -> Self {
        let mut rng = rand::rng();
        let rank = if rng.random_bool(0.2) { EnemyRank::Legendary } else { EnemyRank::Boss };
        Self::spawn_with_type_and_rank(enemy_type, rank)
    }

    pub fn spawn_with_type_and_rank(enemy_type: EnemyType, rank: EnemyRank) -> Self {
        let mut rng = rand::rng();

        // 1) nom depuis le TOML
        let name = pick_name_from_db(enemy_type, rank, &mut rng);

        // 2) stats de base (centralisées ici)
        let base = BaseStats { hp: 10, atk: 3 };
        let (hp, atk) = base.scaled(rank);

        Enemy { name, hp, attack: atk, enemy_type, rank }
    }

    pub fn xp_reward(&self) -> u32 { self.rank.xp_reward() }
}
