use crate::game_mecanics::{Combat, Equipment, Inventory, Player, Stat};

#[derive(Debug, Clone)]
pub struct App {
    screen: MenuScreen,
    game: Option<Game>,
}

#[derive(Debug, Clone)]
pub enum MenuScreen {
    NewGameScreen,
    Quit,
}
#[derive(Debug, Clone)]
pub struct Game {
    player: Player,
    current_screen: GameScreen,
    state: GameState,
    combat: Option<Combat>,
}

#[derive(Debug, Clone)]
pub enum GameScreen {
    StartScreen,
    PauseScreen,
    MainScreen,
    Quit,
}

#[derive(Debug, Clone)]
pub enum GameState {
    Menu,
    Exploration,
    Combat,
    Inventory,
    GameOver,
}

impl App {
    pub fn new() -> Self {
        Self {
            player: None,
            current_screen: CurrentScreen::StartScreen,
            combat: None,
            start_menu_selected: StartMenuItem::NewGame,
        }
    }
    pub fn start(&mut self) {
        self.player = Some(Player {
            name: "Héros".to_string(),
            equipment: Equipment::new(),
            stats: Stat::new(100, 100, 10, 5),
            inventory: Inventory::new(),
        });
        self.current_screen = CurrentScreen::MainScreen;
    }
    pub fn pause(&mut self) {
        self.current_screen = CurrentScreen::PauseScreen;
    }
    pub fn quit(&mut self) {
        self.current_screen = CurrentScreen::Quit;
    }
    pub fn start_menu_next(&mut self) {
        self.start_menu_selected = match self.start_menu_selected {
            StartMenuItem::NewGame => StartMenuItem::Quit,
            StartMenuItem::Quit => StartMenuItem::NewGame,
        };
    }
    pub fn start_menu_previous(&mut self) {
        self.start_menu_selected = match self.start_menu_selected {
            StartMenuItem::NewGame => StartMenuItem::Quit,
            StartMenuItem::Quit => StartMenuItem::NewGame,
        };
    }
    pub fn validate_start_menu(&mut self) {
        match self.start_menu_selected {
            StartMenuItem::NewGame => self.start(),
            StartMenuItem::Quit => self.quit(),
        }
    }
}
