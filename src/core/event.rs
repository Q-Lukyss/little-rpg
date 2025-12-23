use crate::game_mecanics::{Location, Loot};

#[derive(Debug, Clone)]
pub enum Event {
    Exploration(ExplorationEvent),
    CombatEvent(CombatEvent),
    InventoryEvent(InventoryEvent),
    MenuEvent(MenuEvent),
    EnterCombat,
    EncounterEnemy,
    FindLoot(Loot),
    Text(String),
    Quit,
    GameOver,
}

#[derive(Debug, Clone)]
enum Entity {
    Player,
    Enemy,
}

#[derive(Debug, Clone)]
pub enum ExplorationEvent {
    Explore,
    Travel(Location),
    EncounterEnemy,
    FindLoot(Loot),
}

#[derive(Debug, Clone)]
pub enum CombatEvent {
    Attack,
    Defend,
    Block,
    Parry,
    Heal,
    Miss,
    Dodge,
    CriticalHit,
    CriticalMiss,
    Death,
}

#[derive(Debug, Clone)]
pub enum InventoryEvent {
    ItemAdded(Loot),
    ItemRemoved(Loot),
    ItemUsed(Loot),
    ItemEquipped(Loot),
    ItemUnequipped(Loot),
}

#[derive(Debug, Clone)]
pub enum MenuEvent {
    MainMenu,
    Options,
    Save,
    Load,
    Quit,
}
