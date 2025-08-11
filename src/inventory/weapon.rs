#[derive(Debug, Clone)]
pub struct Weapon {
    pub kind: WeaponKind,
    pub base_damage: u32,
}

#[derive(Debug, Clone)]
pub enum WeaponKind {
    Sword,
    Axe,
    Spear,
    LongSword,
}

#[derive(Debug, PartialEq)]
pub enum AttackPattern {
    EveryTurn,
    EveryTwoTurns,
    TwicePerTurn,
    EveryThreeTurns,
}

pub enum Wield {
    OneHand,
    TwoHands
}



impl WeaponKind {
    pub fn attack_pattern(&self) -> AttackPattern {
        match self {
            WeaponKind::Sword     => AttackPattern::EveryTurn,
            WeaponKind::Axe       => AttackPattern::EveryTwoTurns,
            WeaponKind::Spear     => AttackPattern::TwicePerTurn,
            WeaponKind::LongSword => AttackPattern::EveryThreeTurns,
        }
    }

    pub fn get_weapon_wielding(&self) {
        match self {
            WeaponKind::Sword     => Wield::OneHand,
            WeaponKind::Axe       => Wield::OneHand,
            WeaponKind::Spear     => Wield::OneHand,
            WeaponKind::LongSword => Wield::TwoHands,
        }
    }
}

impl Weapon {
    pub fn get_dpt(&self) -> f32 {
        let multiplier = match self.kind.attack_pattern() {
            AttackPattern::EveryTurn      => 1.0,
            AttackPattern::EveryTwoTurns  => 0.5,
            AttackPattern::EveryThreeTurns=> 1.0 / 3.0,
            AttackPattern::TwicePerTurn   => 2.0,
        };
        self.base_damage as f32 * multiplier
    }

    pub fn display(&self) {
        println!(
            "{:?} | Dmg: {} | Pattern: {:?} | DPT: {:.2}",
            self.kind,
            self.base_damage,
            self.kind.attack_pattern(),
            self.get_dpt()
        );
    }
}
