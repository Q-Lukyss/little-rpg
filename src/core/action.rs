#[derive(Debug)]
pub enum Action {
    World(WorldAction),
    Explore,
    Combat(CombatAction),
    Inventory(InventoryAction),
    Quit,
}

#[derive(Debug)]
pub enum WorldAction {
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
    ItemAction(Item),
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
