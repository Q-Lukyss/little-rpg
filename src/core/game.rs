use crate::game_mecanics::Inventory;
use crate::game_mecanics::Stat;

pub struct Game {
    pub player: Player,
    pub state: GameState,
    pub combat: Option<CombatState>,
}

impl Game {
    pub fn new(player_name: &str) -> Self {
        Self {
            player: Player {
                name: player_name.to_string(),
                stats: Stat::new(100, 100, 10, 5),
                inventory: Inventory::new(),
            },
            state: GameState::Exploration,
            combat: None,
        }
    }
}

pub enum GameState {
    Menu,
    Exploration,
    Combat,
    Inventory,
    GameOver,
}

pub enum CombatState {
    Start,
    PlayerTurn,
    EnemyTurn,
    End,
}

pub struct Combat {
    pub player: Player,
    pub opponents: Vec<Enemy>,
    pub turn: u32,
}

pub struct Player {
    pub name: String,
    pub stats: Stat,
    pub inventory: Inventory,
}

pub struct Enemy {
    pub name: Option<String>,
    pub stats: Stat,
    pub inventory: Inventory,
}

// pub fn apply(game: &mut Game, action: Action) -> Vec<Event> {
//     match (&game.game_state, action) {
//         (GameState::ScreenMenu, Action::Screen(screen_action)) => {
//             screen::apply_screen_choice(game, screen_action)
//         }
//         (GameState::Menu, Action::GoTo(dest)) => flow::apply_menu_choice(game, dest),
//         (GameState::Exploration, Action::Combat(ca)) => combat::apply_combat_action(game, ca),
//         (GameState::Combat, Action::Combat(ca)) => combat::apply_combat_action(game, ca),
//         (GameState::Inventory, Action::Combat(ca)) => combat::apply_combat_action(game, ca),
//         (GameState::GameOver, Action::Combat(ca)) => combat::apply_combat_action(game, ca),
//         // ...
//         _ => vec![Event::Text("Action impossible ici.".into())],
//     }
// }

// // domain/combat.rs
// pub fn apply_combat_action(game: &mut Game, action: CombatAction) -> Vec<Event> {
//     let mut ev = vec![];

//     // 1) effet de l’action joueur
//     // 2) si ennemi vivant → action ennemi (pattern)
//     // 3) check fin combat → xp/loot → change mode

//     ev
// }
