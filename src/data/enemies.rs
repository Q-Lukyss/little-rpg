use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs, path::Path};

use crate::enemy::{EnemyRank, EnemyType};

#[derive(Deserialize)]
pub struct EnemyRow {
    #[serde(rename = "type")]
    pub kind: EnemyType,
    pub rank: EnemyRank,
    pub names: Vec<String>,
}

#[derive(Deserialize)]
struct EnemyFile {
    enemies: Vec<EnemyRow>,
}

// Charge assets/enemies.toml une seule fois
pub static ENEMY_DB: Lazy<Vec<EnemyRow>> = Lazy::new(|| {
    // Embarque le contenu du fichier dans une constante à la compilation
    const RAW_TOML: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/enemies.toml"));
    // Parse le TOML
    let f: EnemyFile = toml::from_str(RAW_TOML).expect("TOML invalide");
    f.enemies
});

/// Retourne un nom aléatoire ou None si non trouvé
pub fn pick_name_from_db(
    t: EnemyType,
    r: EnemyRank,
    rng: &mut impl rand::Rng,
) -> Option<String> {
    // rand 0.9 -> IndexedRandom ; rand 0.8 -> SliceRandom
    use rand::prelude::IndexedRandom;
    ENEMY_DB
        .iter()
        .find(|e| e.kind == t && e.rank == r)
        .and_then(|row| row.names.choose(rng).cloned())
}
