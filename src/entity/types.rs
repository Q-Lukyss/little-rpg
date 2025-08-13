use crate::inventory::{Weapon, weapon::OffHand, Item};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Entity {
    pub name: Option<String>,
    pub stats: Stats,
    pub equipment: Equipment,
    pub buffs: Vec<ActiveBuff>,
    pub kind: Kind,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub hp: u32,
    pub max_hp: u32,
    pub attack: i32,
    pub speed: i32,
    pub defense: i32,
    pub crit: f32,       // 0.0..=1.0
    pub level: u8,
    pub xp: Option<u32>,
    pub xp_to_level: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatsType {
    Attack,
    Speed,
    Defense,
    Hp,
    MaxHp,
    Crit,
}

impl Stats {
    pub fn is_alive(&self) -> bool { self.hp > 0 }
    pub fn take_damage(&mut self, raw: i32) {
        let dmg = raw.max(0) as u32;
        self.hp = self.hp.saturating_sub(dmg);
    }
}

#[derive(Debug, Clone)]
pub struct Equipment {
    pub main: Weapon,              // toujours présent
    pub off: Option<OffHand>,      // bouclier / arme secondaire
}

#[derive(Debug, Clone)]
pub struct ActiveBuff {
    pub stat: StatsType,
    pub value: u32,
    pub remaining_turns: u8,
}

#[derive(Debug, Clone)]
pub enum Kind {
    Player(PlayerData),
    Enemy(EnemyData),
}

#[derive(Debug, Clone)]
pub struct PlayerData {
    pub inventory: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct EnemyData {
    pub enemy_type: EnemyType,
    pub rank: EnemyRank,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum EnemyRank {
    Lambda,
    Named,
    Elite,
    Boss,
    Legendary,
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum EnemyType {
    Gobelin,
    Human,
    Skeleton,
    Vampire,
    Demon,
    Dragon,
    Orc,
}

impl EnemyType {
    pub fn as_str(&self) -> &str {
        match self {
            EnemyType::Gobelin  => "Gobelin",
            EnemyType::Skeleton => "Squelette",
            EnemyType::Human    => "Bandit",
            EnemyType::Vampire  => "Vampire",
            EnemyType::Demon    => "Démon",
            EnemyType::Dragon   => "Dragon",
            EnemyType::Orc      => "Orc",
        }
    }
}
