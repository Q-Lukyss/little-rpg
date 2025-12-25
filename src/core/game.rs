use crate::game_mecanics::{Combat, Equipment, Inventory, Player, Stat};

#[derive(Debug, Clone)]
pub struct Game {
    pub player: Player,
    pub state: GameState,
    pub combat: Option<Combat>,
}

#[derive(Debug, Clone)]
pub enum GameState {
    Menu,
    Exploration,
    Combat,
    Inventory,
    GameOver,
}

impl Game {
    pub fn new(player_name: &str) -> Self {
        Self {
            player: Player {
                name: player_name.to_string(),
                equipment: Equipment::new(),
                stats: Stat::new(100, 100, 10, 5),
                inventory: Inventory::new(),
            },
            state: GameState::Exploration,
            combat: None,
        }
    }
}
