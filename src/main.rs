mod core;
mod game_mecanics;
mod ui;

use std::error::Error;
use std::io::stdout;

use ratatui::Terminal;
use ratatui::crossterm::event::{self, Event};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::{Backend, CrosstermBackend};

use crate::core::App;

use crate::core::app::CurrentScreen;
use crate::ui::ratatui::Ratatui;

fn main() -> Result<(), Box<dyn Error>> {
    // 1) On passe le terminal en mode "jeu/TUI"
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    // 2) On crée ratatui
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    // 3) On lance l'app et on stocke le résultat
    let result = run(&mut terminal, &mut app);

    // 4) IMPORTANT : on remet le terminal en état normal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // 5) On retourne le résultat de l'app
    result
}

fn run(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    let mut ui = Ratatui;

    loop {
        terminal.draw(|f| ui.render(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match app.current_screen {
                CurrentScreen::StartScreen => handle_start_screen_input(&key, app),
                CurrentScreen::MainScreen => todo!("MainScreen"),
                CurrentScreen::PauseScreen => todo!("PauseScreen"),
                CurrentScreen::Quit => {}
            }
        }

        if matches!(app.current_screen, CurrentScreen::Quit) {
            break;
        }
    }

    Ok(())
}

fn handle_start_screen_input(key: &event::KeyEvent, app: &mut App) {
    match key.code {
        event::KeyCode::Up => app.start_menu_previous(),
        event::KeyCode::Down => app.start_menu_next(),
        event::KeyCode::Enter => app.validate_start_menu(),
        event::KeyCode::Char('q') | event::KeyCode::Esc => app.quit(),
        _ => {}
    }
}
