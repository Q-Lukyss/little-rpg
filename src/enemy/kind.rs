use serde::{Serialize, Deserialize};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum EnemyType { Gobelin, Skeleton, Human }

impl EnemyType {
    pub fn as_str(&self) -> &str {
        match self {
            EnemyType::Gobelin  => "Gobelin",
            EnemyType::Skeleton => "Squelette",
            EnemyType::Human    => "Bandit",
        }
    }
}
