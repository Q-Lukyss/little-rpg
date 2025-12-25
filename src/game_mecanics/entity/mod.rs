use crate::{
    core::CombatAction,
    game_mecanics::{Armor, Handling, Inventory, Shield, Stat, Trinket, Weapon, WeaponType},
};

#[derive(Debug, Clone)]
pub enum Entity {
    Player(Player),
    Enemy(Enemy),
}
#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub stats: Stat,
    pub equipment: Equipment,
    pub inventory: Inventory,
}
#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: Option<String>,
    pub stats: Stat,
    pub equipment: Equipment,
    pub attack_pattern: Vec<CombatAction>,
    pub level: u32,
    pub race: EnemyRace,
}

#[derive(Debug, Clone)]
pub enum MainHand {
    Weapon(Weapon),
}

#[derive(Debug, Clone)]
pub enum OffHand {
    Weapon(Weapon),
    Shield(Shield),
}

#[derive(Debug, Clone)]
pub enum OffHandState {
    Empty,
    Equipped(OffHand),
    LockedByTwoHanded,
}

#[derive(Debug, Clone)]
pub struct Equipment {
    pub main_hand: Option<MainHand>,
    pub off_hand: OffHandState,
    pub armor: Option<Armor>,
    pub trinkets: [Option<Trinket>; 5], // possibilité d'équiper jusque 5 trinket
}

impl Equipment {
    pub fn new() -> Self {
        Self {
            main_hand: None,
            off_hand: OffHandState::Empty,
            armor: None,
            trinkets: [None, None, None, None, None],
        }
    }
}

impl Enemy {
    pub fn new(
        name: Option<String>,
        stats: Stat,
        equipment: Equipment,
        attack_pattern: Vec<CombatAction>,
        level: u32,
        race: EnemyRace,
    ) -> Self {
        Self {
            name,
            stats,
            equipment,
            attack_pattern,
            level,
            race,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.stats.hp <= 0
    }

    pub fn new_gobelin() -> Self {
        Self::new(
            None,
            Stat::new(10, 10, 5, 2),
            Equipment {
                main_hand: None,
                off_hand: OffHandState::Empty,
                armor: None,
                trinkets: [None, None, None, None, None],
            },
            vec![CombatAction::Attack],
            1,
            EnemyRace::Goblin,
        )
    }
    pub fn new_armored_gobelin() -> Self {
        Self::new(
            None,
            Stat::new(10, 10, 10, 10),
            Equipment {
                main_hand: Some(MainHand::Weapon(Weapon::new(
                    "Rapière gobeline".into(),
                    "l'épée classique ce ces monstres vertdatre".into(),
                    10,
                    WeaponType::Sword,
                ))),
                off_hand: OffHandState::Empty,
                armor: Some(Armor::new(
                    "Armure gobeline".into(),
                    "armure de base".into(),
                    5,
                )),
                trinkets: [None, None, None, None, None],
            },
            vec![CombatAction::Attack],
            1,
            EnemyRace::Goblin,
        )
    }
}
#[derive(Debug, Clone)]
pub enum EnemyRace {
    Goblin,
    Skeleton,
    Human,
    Orc,
    Demon,
    Vampire,
}
