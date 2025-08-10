use crate::{enemy::Enemy};

pub enum GameState {
    Menu,
    Exploration,
    Combat(Enemy),
    GameOver,
    Inventory
}