pub struct Game {
    pub player: Player,
    pub game_state: GameState,
    pub combat: Option<CombatState>,
}

pub enum GameState {
    Menu,
    Exploration,
    Combat,
    Inventory,
    GameOver,
}
