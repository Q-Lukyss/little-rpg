use crate::{
    core::Game,
    game_mecanics::{Combat, Enemy, Item, Location, Loot},
};

#[derive(Debug, Clone)]
pub enum Event {
    Exploration(ExplorationEvent),
    CombatEvent(CombatEvent),
    InventoryEvent(InventoryEvent),
    MenuEvent(MenuEvent),
    EnterCombat,
    FindLoot(Loot),
    Text(String),
    Quit,
    GameOver,
}

#[derive(Debug, Clone)]
pub enum ExplorationEvent {
    Explore,
    Travel(Location),
    EncounterEnemy(Vec<Enemy>),
    FindLoot(Loot),
}

#[derive(Debug, Clone)]
pub enum CombatEvent {
    Attack,
    Block,
    Parry,
    Flee,
    UseItem(Item),
    EnemyDefeated(Enemy),
    Report(Combat),
    ResolveTurn,
}

#[derive(Debug, Clone)]
pub enum InventoryEvent {
    ReadItemDetails(Item),
    EquipItem(Item),
    UnequipItem(Item),
    UseItem(Item),
}

#[derive(Debug, Clone)]
pub enum MenuEvent {
    MainMenu,
    Save(Game),
    Load(Game),
    Quit,
}
