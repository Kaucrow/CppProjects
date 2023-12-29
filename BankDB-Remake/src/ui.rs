use ratatui::{
    layout::{Layout, Direction, Rect, Constraint},
    prelude::{Alignment, Frame},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph}
};
use std::sync::{Arc, Mutex};
use crate::model::{ App, Screen };

pub fn render(app: &mut Arc<Mutex<App>>, f: &mut Frame) {
    match app.lock().unwrap().curr_screen {
        Screen::Login => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(2),
                    Constraint::Min(6),
                    Constraint::Max(999),
                ])
                .split(centered_rect(30, 50, f.size()));
            
            let input_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let input = Paragraph::new(Text::from(
                "Input your name"
            ))
            .block(input_block)
            .alignment(Alignment::Center);

            f.render_widget(input, chunks[1]);

            let title_block = Block::default();

            let title = Paragraph::new(Text::from(
                "BankDB"
            ))
            .block(title_block)
            .alignment(Alignment::Center);

            f.render_widget(title, chunks[0]);
        },
    }
    /*f.render_widget(Paragraph::new(format!(
        "
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
            Press `j` and `k` to increment and decrement the counter respectively.\n\
            Counter: {}
        ",
        app.counter
    ))
    .block(
        Block::default()
        .title("Counter App")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded),
    )
    .style(Style::default().fg(Color::Yellow))
    .alignment(Alignment::Center),
    f.size(),
    )*/
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(layout[1])[1]
}