use rand::Rng;
use super::EnemyRank;

/// Structure qui contient la probabilité (en %) d'apparition de chaque rang
#[derive(Clone, Copy)]
pub struct RankRates {
    pub lambda: u8,
    pub named: u8,
    pub elite: u8,
    pub boss: u8,
    pub legendary: u8,
}

/// Pondérations par défaut
pub const DEFAULT_RATES: RankRates = RankRates {
    lambda: 70,    // 70 % de chance
    named: 20,     // 20 % de chance
    elite: 9,      // 9 % de chance
    boss: 1,       // 1 % de chance
    legendary: 0,  // 0 % par défaut
};

/// Choisit un rang aléatoirement en fonction des pondérations données
pub fn roll_rank(rng: &mut impl Rng, w: RankRates) -> EnemyRank {
    let total = (w.lambda as u16 + w.named as u16 + w.elite as u16 + w.boss as u16 + w.legendary as u16) as u16;
    let x = rng.random_range(0..total) as u16;
    let mut acc = 0u16;

    for (rank, weight) in [
        (EnemyRank::Lambda,    w.lambda),
        (EnemyRank::Named,     w.named),
        (EnemyRank::Elite,     w.elite),
        (EnemyRank::Boss,      w.boss),
        (EnemyRank::Legendary, w.legendary),
    ] {
        acc += weight as u16;
        if x < acc { return rank; }
    }

    EnemyRank::Lambda // fallback (ne devrait jamais arriver)
}
