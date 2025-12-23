use crate::game_mecanics::{Location, Loot};

#[derive(Debug)]
pub enum Action {
    Menu(MenuAction),
    Exploration(ExplorationAction),
    Combat(CombatAction),
    Inventory(InventoryAction),
    Quit,
}

#[derive(Debug)]
pub enum ExplorationAction {
    Explore,
    Travel(Location),
    EncounterEnemy,
    FindLoot(Loot),
}

#[derive(Debug)]
pub enum MenuAction {
    Interact,
    Craft,
    Save,
}

#[derive(Debug)]
pub enum CombatAction {
    Attack,
    Defend,
    Parry,
    Block,
    Flee,
    UseItem(Item),
}

#[derive(Debug)]
pub enum InventoryAction {
    View,
    ItemAction(ItemAction),
    Unequip(Item),
}

#[derive(Debug)]
pub enum ItemAction {
    Use(Item),
    Equip(Item),
    Unequip(Item),
    Read(Item),
}

#[derive(Debug)]
pub enum Item {
    Weapon(WeaponType),
    Armor,
    Consumable(ConsumableType),
    Shield,
    Trinket,
}

#[derive(Debug)]
pub enum ConsumableType {
    Potion,
    Elixir,
}

#[derive(Debug)]
pub enum WeaponType {
    Sword,
    Axe,
    Spear,
    LongSword,
}
