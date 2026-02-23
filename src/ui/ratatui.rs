use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
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

        let div = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        let title = Paragraph::new(Text::styled(
            "Little RPG | by Quentin 'Lukyss' Lachery",
            Style::default()
                .fg(Color::Indexed(202))
                .add_modifier(Modifier::BOLD),
        ))
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

        frame.render_widget(title, div[0]);

        let new_game_style = match &app.start_menu_selected {
            StartMenuItem::NewGame => Style::default()
                .fg(Color::Black)
                .bg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            _ => Style::default().fg(Color::White),
        };

        let quit_style = match &app.start_menu_selected {
            StartMenuItem::Quit => Style::default()
                .fg(Color::Black)
                .bg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
            _ => Style::default().fg(Color::White),
        };

        let menu_text = Text::from(vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                match &app.start_menu_selected {
                    StartMenuItem::NewGame => ">  Nouvelle partie",
                    StartMenuItem::Quit => "  Nouvelle partie",
                },
                new_game_style,
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                match &app.start_menu_selected {
                    StartMenuItem::Quit => ">  Quitter",
                    StartMenuItem::NewGame => "  Quitter",
                },
                quit_style,
            )]),
            Line::from(""),
        ]);

        let menu = Paragraph::new(menu_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Menu")
                    .border_style(Style::default().fg(Color::Magenta)),
            )
            .alignment(Alignment::Center);

        frame.render_widget(menu, div[1]);
    }

    fn render_main_screen(&mut self, frame: &mut Frame, _app: &App) {
        let block = Block::default().borders(Borders::ALL).title("Main Screen");
        frame.render_widget(block, frame.area());
    }

    fn render_pause_screen(&mut self, frame: &mut Frame, _app: &App) {
        let block = Block::default().borders(Borders::ALL).title("Pause");
        frame.render_widget(block, frame.area());
    }
}
