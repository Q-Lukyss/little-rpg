mod core;
mod game_mecanics;
mod ui;

use std::error::Error;
use std::io::stdout;

use ratatui::Terminal;
use ratatui::crossterm::event::{self, Event};
use ratatui::prelude::{Backend, CrosstermBackend};

use crate::core::App;

use crate::core::app::CurrentScreen;
use crate::ui::ratatui::Ratatui;

fn main() -> Result<(), Box<dyn Error>> {
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    run(&mut terminal, &mut app);

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<(), Box<dyn Error>>
where
    std::io::Error: From<<B as Backend>::Error>,
{
    let mut ui = Ratatui;
    loop {
        terminal.draw(|f| ui.render(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::StartScreen => {
                    handle_start_screen_input(&key, app);
                }
                CurrentScreen::MainScreen => {
                    todo!("MainScreen")
                }
                CurrentScreen::PauseScreen => {
                    todo!("PauseScreen")
                }
                CurrentScreen::Quit => todo!("Quit App"),
            }
        }
    }
}

fn handle_start_screen_input(key: &event::KeyEvent, app: &mut App) {
    match key.code {
        event::KeyCode::Up => {}
        event::KeyCode::Down => {}
        event::KeyCode::Enter => {}
        _ => {}
    }
}
