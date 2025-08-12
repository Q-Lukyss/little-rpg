use crate::{enemy::Enemy};

pub enum GameState {
    Menu,
    Exploration,
    Combat(Vec<Enemy>),
    GameOver,
    Inventory
}