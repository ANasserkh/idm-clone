use std::{cell, str::Lines};

use crossterm::{style, terminal::window_size};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table},
    Frame,
};

use crate::app::{self, App, CurrentScreen};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    render_header(f, chunks[0]);

    render_body(f, app, chunks[1]);

    if app.current_screen == CurrentScreen::AddDialog {
        render_dialog(f, app);
    }

    render_footer(f, chunks[2], app);
}

fn render_body(f: &mut Frame, app: &App, body: Rect) {
    let columns = [Constraint::Max(2), Constraint::Min(1)];

    let rows = app
        .links
        .iter()
        .enumerate()
        .map(|(index, link)| Row::new(link.to_row(index)))
        .collect::<Vec<Row>>();

    let block = Block::default()
        .title("files list")
        .borders(ratatui::widgets::Borders::ALL);

    let text = Table::new(rows, columns).block(block);

    f.render_widget(text, body);
}

fn render_dialog(f: &mut Frame, app: &App) {
    let popup_block = Block::default()
        .title("Enter download link")
        .borders(Borders::NONE)
        .style(Style::default().bg(Color::DarkGray));

    let area = centered_rect(60, 10, f.size());

    f.render_widget(popup_block, area);

    let mut link_input = Block::default().title("link").borders(Borders::ALL);

    let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

    let key_text = Paragraph::new(app.input.clone()).block(link_input);
    f.render_widget(key_text, area);
}

fn render_header(f: &mut Frame, header_layout: Rect) {
    let title_block = Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .style(Style::default());

    let text = Text::from("IDM Clone TUI".blue().bold());
    let header_title = Paragraph::new(text).block(title_block);

    f.render_widget(header_title, header_layout);
}

fn render_footer(f: &mut Frame, footer_layout: Rect, app: &App) {
    let title_block = Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .style(Style::default());
    let lines: Line = match app.current_screen {
        CurrentScreen::Main => vec![
            "(Q) Quit the terminal".into(),
            " / ".into(),
            "(A) Add new download link".into(),
        ]
        .into(),
        CurrentScreen::AddDialog => vec![
            "(ESC) Close the dialog".into(),
            " / ".into(),
            "(Enter) Submit".into(),
        ]
        .into(),
    };

    let header_title = Paragraph::new(lines).block(title_block);
    f.render_widget(header_title, footer_layout);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
