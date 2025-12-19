pub enum Action {
    World(WorldAction),
    Combat(CombatAction),
    Inventory(InventoryAction),
}

pub enum WorldAction {
    Interact,
    Craft,
}

pub enum CombatAction {
    Attack,
    Defend,
    Parry,
    Block,
    Flee,
    UseItem(Item),
}

pub enum InventoryAction {
    View,
    ItemAction(Item),
    Unequip(Item),
}

pub enum ItemAction {
    Use(Item),
    Equip(Item),
    Unequip(Item),
    Read(Item),
}

pub enum Item {
    Weapon(WeaponType),
    Armor,
    Consumable(ConsumableType),
    Shield,
    Trinket,
}

pub enum ConsumableType {
    Potion,
    Elixir,
}

pub enum WeaponType {
    Sword,
    Axe,
    Spear,
    LongSword,
}
