pub enum Event {
    EnteredCombat,
    Text(String),
    Hit {
        source: Actor,
        target: Actor,
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
        item: Item,
    },
    XpGained {
        amount: u32,
    },
    CombatEnded {
        victory: bool,
    },
}
