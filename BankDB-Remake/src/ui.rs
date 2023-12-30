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
    let app_lock = app.lock().unwrap();

    match app_lock.curr_screen {
        Screen::Login => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Min(3),
                    Constraint::Min(3),
                    Constraint::Max(999),
                ])
                .split(centered_rect(25, 40, f.size()));
            
            let title_block = Block::default();

            let title = Paragraph::new(Text::from(
                "BankDB"
            ))
            .block(title_block)
            .alignment(Alignment::Center);

            f.render_widget(title, chunks[0]);

            let width = chunks[0].width.max(3) - 3;
            let scroll = app_lock.input.value.visual_scroll(width as usize - "* Name: ".len());
            
            let name_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let input = Paragraph::new(Text::from(Line::from(vec![
                Span::styled("* Name: ", Style::default().fg(Color::Yellow)),
                Span::raw(app_lock.input.value.value().to_string())
            ])))
            .block(name_block)
            .scroll((0, scroll as u16));

            f.render_widget(input, chunks[1]);

            f.set_cursor(chunks[1].x
                            + ((app_lock.input.value.visual_cursor()).max(scroll) - scroll) as u16
                            + "* Name: ".len() as u16
                            + 1,
                        chunks[1].y + 1,
                        );
            /*
            let password_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded);

            let input = Paragraph::new(Text::from(Line::from(vec![
                Span::styled("* Password: ", Style::default().fg(Color::Yellow)),
                Span::raw(app_lock.input.value.value().to_string())
            ])))
            .block(password_block)
            .scroll((0, scroll as u16));

            f.render_widget(input, chunks[2]);

            f.set_cursor(chunks[2].x
                            + ((app_lock.input.value.visual_cursor()).max(scroll) - scroll) as u16
                            + "* Password: ".len() as u16
                            + 1,
                        chunks[2].y + 1,
                        );*/
        },
    }
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