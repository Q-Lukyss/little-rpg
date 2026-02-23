use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::core::app::{App, CurrentScreen, StartMenuItem};

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
        let area = frame.area();

        // Layout global : titre / centre / footer
        let div = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // titre
                Constraint::Min(1),    // zone centrale
                Constraint::Length(3), // footer
            ])
            .split(area);

        // Titre
        let title = Paragraph::new(Text::styled(
            "Little RPG",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ))
        .block(Block::default().borders(Borders::ALL).title("Accueil"))
        .alignment(Alignment::Center);

        frame.render_widget(title, div[0]);

        // Zone centrale -> on place un "popup" menu centré
        let menu_area = Self::centered_rect(40, 7, div[1]); // largeur=40, hauteur=7
        frame.render_widget(Clear, menu_area); // nettoie le fond derrière (utile plus tard)

        let new_game_style = match &app.start_menu_selected {
            StartMenuItem::NewGame => Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
            _ => Style::default().fg(Color::White),
        };

        let quit_style = match &app.start_menu_selected {
            StartMenuItem::Quit => Style::default()
                .fg(Color::Black)
                .bg(Color::Green)
                .add_modifier(Modifier::BOLD),
            _ => Style::default().fg(Color::White),
        };

        let menu_text = Text::from(vec![
            Line::from(""),
            Line::from(vec![Span::styled("  Nouvelle partie", new_game_style)]),
            Line::from(""),
            Line::from(vec![Span::styled("  Quitter", quit_style)]),
            Line::from(""),
        ]);

        let menu = Paragraph::new(menu_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Menu")
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center);

        frame.render_widget(menu, menu_area);

        // Footer
        let footer = Paragraph::new(Text::styled(
            "By Quentin \"Lukyss\" Lachery",
            Style::default().fg(Color::DarkGray),
        ))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

        frame.render_widget(footer, div[2]);
    }

    fn render_main_screen(&mut self, frame: &mut Frame, _app: &App) {
        let block = Block::default().borders(Borders::ALL).title("Main Screen");
        frame.render_widget(block, frame.area());
    }

    fn render_pause_screen(&mut self, frame: &mut Frame, _app: &App) {
        let block = Block::default().borders(Borders::ALL).title("Pause");
        frame.render_widget(block, frame.area());
    }

    fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(height),
                Constraint::Min(0),
            ])
            .split(area);

        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(width),
                Constraint::Min(0),
            ])
            .split(vertical[1]);

        horizontal[1]
    }
}
