use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::core::app::{App, CurrentScreen};

pub struct Ratatui;

impl Ratatui {
    pub fn render(&mut self, frame: &mut Frame, app: &App) {
        match &app.current_screen {
            CurrentScreen::StartScreen => {
                self.render_start_screen(frame, app);
            }
            CurrentScreen::MainScreen => {
                self.render_main_screen(frame, app);
            }
            CurrentScreen::PauseScreen => {
                self.render_pause_screen(frame, app);
            }
            CurrentScreen::Quit => {}
        }
    }

    fn render_start_screen(&mut self, frame: &mut Frame, app: &App) {
        let div = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "Create New Json",
            Style::default().fg(Color::Green),
        ))
        .block(title_block)
        .centered();

        
        
        
        frame.render_widget(title, div[0]);

        let footer_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let footer = Paragraph::new(Text::styled(
            "By Quentin 'Lukyss' Lachery",
            Style::default().fg(Color::Green),
        ))
        .block(footer_block)
        .centered();

        frame.render_widget(footer, div[2]);
    }

    fn render_main_screen(&mut self, frame: &mut Frame, app: &App) {
        // Render main screen
    }

    fn render_pause_screen(&mut self, frame: &mut Frame, app: &App) {
        // Render pause screen
    }
}
