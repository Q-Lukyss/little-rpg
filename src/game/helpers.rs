use std::io::{self, Read};

use crate::inventory::{AttackPattern, WeaponKind};

pub fn read_line_trimmed() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn can_attack_this_turn(turn: &u32, weapon_kind: &WeaponKind) -> bool {
    match weapon_kind.attack_pattern() {
        AttackPattern::EveryTurn => true,
        AttackPattern::EveryTwoTurns => turn % 2 == 0,
        AttackPattern::EveryThreeTurns => turn % 3 == 0,
        AttackPattern::TwicePerTurn => true
    }
}