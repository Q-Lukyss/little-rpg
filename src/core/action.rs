use crate::game_mecanics::{Item, Location, Loot};

#[derive(Debug, Clone)]
pub enum Action {
    Menu(MenuAction),
    Exploration(ExplorationAction),
    Combat(CombatAction),
    Inventory(InventoryAction),
    Quit,
}

#[derive(Debug, Clone)]
pub enum ExplorationAction {
    Explore,
    Travel(Location),
    EncounterEnemy,
    FindLoot(Loot),
}

#[derive(Debug, Clone)]
pub enum CombatAction {
    Attack,
    Parry,
    Block,
    Flee,
    UseItem(Item),
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    Interact,
    Craft,
    Save,
}

#[derive(Debug, Clone)]
pub enum InventoryAction {
    View,
    ItemAction(ItemAction),
    Unequip(Item),
}

#[derive(Debug, Clone)]
pub enum ItemAction {
    Use(Item),
    Equip(Item),
    Unequip(Item),
    Read(Item),
}
