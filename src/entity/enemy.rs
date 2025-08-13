use super::types::{Entity, Stats, Equipment, Kind, EnemyData, EnemyType, EnemyRank};
use super::traits::{AttackProfile, HasStats};
use super::combat::resolve_attack;

use crate::inventory::{Weapon, WeaponKind};
use crate::data::enemies::pick_name_from_db;
use rand::Rng;

// ---------------------------------------------------------
// Probabilités de rang (ex- spawn_rate.rs)
// ---------------------------------------------------------

#[derive(Clone, Copy)]
pub struct RankRates {
    pub lambda: u8,
    pub named: u8,
    pub elite: u8,
    pub boss: u8,
    pub legendary: u8,
}

pub const DEFAULT_RATES: RankRates = RankRates {
    lambda: 70,
    named: 20,
    elite: 9,
    boss: 1,
    legendary: 0,
};

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
    EnemyRank::Lambda
}

// ---------------------------------------------------------
// Base stats + scaling par rang
// ---------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct BaseStats { hp: i32, atk: i32 }

impl BaseStats {
    fn scaled(self, rank: EnemyRank) -> (u32, i32) {
        let m = rank.stat_multiplier();
        let hp  = ((self.hp as f32)  * m).round().max(1.0) as u32;
        let atk = ((self.atk as f32) * m).round().max(0.0) as i32;
        (hp, atk)
    }
}

// ---------------------------------------------------------
// Spawners -> retournent des Entity de Kind::Enemy
// ---------------------------------------------------------

pub fn spawn(enemy_type: EnemyType) -> Entity {
    let mut rng = rand::rng();
    let rank = roll_rank(&mut rng, DEFAULT_RATES);
    spawn_with_type_and_rank(enemy_type, rank)
}

pub fn spawn_multiple(enemy_type: EnemyType, count: usize) -> Vec<Entity> {
    (0..count).map(|_| spawn(enemy_type)).collect()
}

pub fn spawn_boss(enemy_type: EnemyType) -> Entity {
    let mut rng = rand::rng();
    let rank = if rng.random_bool(0.2) { EnemyRank::Legendary } else { EnemyRank::Boss };
    spawn_with_type_and_rank(enemy_type, rank)
}

pub fn spawn_with_type_and_rank(enemy_type: EnemyType, rank: EnemyRank) -> Entity {
    let mut rng = rand::rng();

    // 1) nom depuis le TOML
    let name = pick_name_from_db(enemy_type, rank, &mut rng);

    // 2) stats de base (centralisées ici)
    let base = BaseStats { hp: 10, atk: 3 };
    let (hp, atk) = base.scaled(rank);

    // Arme “naturelle” à 0 dégât pour garder attack_value() ~ stats.attack.
    Entity {
        name,
        stats: Stats {
            hp, max_hp: hp,
            attack: atk,
            defense: 0,
            crit: 0.0,
            level: 1,
            xp: None,
            xp_to_level: None,
        },
        equipment: Equipment {
            // Si tu as WeaponKind::NaturalClaws, remplace ici :
            // main: Weapon { kind: WeaponKind::NaturalClaws, base_damage: 0, dmg_reduction_parry: 0 },
            main: Weapon { kind: WeaponKind::Sword, base_damage: 0, dmg_reduction_parry: 0 },
            off: None,
        },
        buffs: vec![],
        kind: Kind::Enemy(EnemyData { enemy_type, rank }),
    }
}

// ---------------------------------------------------------
// Vue "Enemy" : helpers côté IA / combat / récompenses
// ---------------------------------------------------------

pub struct EnemyView<'a> {
    pub e: &'a mut Entity,
}

impl Entity {
    /// Retourne une vue mutable si (et seulement si) l'entité est un ennemi.
    pub fn as_enemy_mut(&mut self) -> Option<EnemyView<'_>> {
        match self.kind {
            Kind::Enemy(_) => Some(EnemyView { e: self }),
            _ => None,
        }
    }
}

impl<'a> EnemyView<'a> {
    /// Dégâts infligés à une cible (wrap de resolve_attack).
    pub fn attack_entity(&self, defender: &mut Entity) -> i32 {
        resolve_attack(self.e, defender)
    }

    /// Se faire frapper par un attaquant (affiche et applique les dégâts).
    pub fn defend_against(&mut self, attacker: &Entity) -> i32 {
        let raw = attacker.attack_value() - self.e.defense_value();
        let dmg = raw.max(0);
        if dmg == 0 {
            println!("L'ennemi bloque entièrement l'attaque !");
        } else {
            println!("L'ennemi encaisse {} dégâts.", dmg);
        }
        self.e.stats.take_damage(dmg);
        dmg
    }

    /// Récompense d’XP (équiv. Enemy::xp_reward()).
    pub fn xp_reward(&self) -> u32 {
        match &self.e.kind {
            Kind::Enemy(EnemyData { rank, .. }) => rank.xp_reward(),
            _ => 0,
        }
    }

    /// Accès pratique au type/rang si besoin
    pub fn meta(&self) -> Option<(EnemyType, EnemyRank)> {
        match &self.e.kind {
            Kind::Enemy(EnemyData { enemy_type, rank }) => Some((*enemy_type, *rank)),
            _ => None,
        }
    }
}
