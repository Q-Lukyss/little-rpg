use crate::entity::Entity;

pub enum GameState {
    Menu,
    Exploration,
    Combat(Vec<Entity>),
    GameOver,
    Inventory,
}
