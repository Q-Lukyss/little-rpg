use crate::game_mecanics::Loot;

pub enum Event {
    EnterCombat,
    EncounterEnemy,
    FindLoot(Loot),
    Text(String),
    Hit {
        source: Entity,
        target: Entity,
        dmg: u32,
    },
    Blocked {
        amount: u32,
    },
    Parry {
        blocked_amount: u32,
        retaliation_amount: u32,
    },
    Healed {
        amount: u32,
    },
    LootGained {
        item: Loot,
    },
    XpGained {
        amount: u32,
    },
    CombatEnded {
        victory: bool,
    },
}

enum Entity {
    Player,
    Enemy,
}
