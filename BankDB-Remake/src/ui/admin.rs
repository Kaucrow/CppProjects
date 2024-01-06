use ratatui::{
    layout::{Layout, Direction, Rect, Constraint, Margin},
    prelude::{Alignment, Frame},
    style::{Color, Style, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, List, BorderType, Borders, Paragraph, Clear}
};
use std::sync::{Arc, Mutex};
use crate::{
    model::app::{
        App,
        Popup,
        Filter,
    },
    ui::common_fn::{
        centered_rect,
        percent_x,
        percent_y,
        clear_chunks
    }
};

pub fn render(app: &mut Arc<Mutex<App>>, f: &mut Frame) {
    let mut app_lock = app.lock().unwrap();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(100),
            Constraint::Min(3)
        ])
        .split(centered_rect(
            percent_x(f, 2.0),
            percent_y(f, 1.5),
            f.size()
        ));
    
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70)
        ])
        .split(chunks[0]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Percentage(100)
        ])
        .split(main_chunks[0].inner(&Margin::new(2, 1)));
    
    let help_block = Block::default().borders(Borders::TOP);
    let help = Paragraph::new(Line::from(
        Span::raw(app_lock.help_text
    )))
    .block(help_block);

    f.render_widget(help, chunks[1]);

    let admin_title = Paragraph::new(Line::from(vec![
        Span::raw(" Login: "),
        Span::styled("Admin", Style::default().fg(Color::Yellow))
    ]))
    .block(Block::default().borders(Borders::BOTTOM));

    f.render_widget(Block::default().borders(Borders::ALL), main_chunks[0]);
    f.render_widget(admin_title, left_chunks[0]);

    f.render_widget(Block::default().borders(Borders::ALL), main_chunks[1]);
    
    let actions = List::new(app_lock.admin.actions.clone()).highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(actions, left_chunks[1], &mut app_lock.admin.action_list_state);
    
    match app_lock.active_popup {
        Some(Popup::FilterClients) => {
            let popup_rect = centered_rect(
                percent_x(f, 1.3),
                percent_y(f, 1.0),
                f.size()
            );

            f.render_widget(Clear, popup_rect);

            let popup_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(35),
                    Constraint::Percentage(65),
                ])
                .split(popup_rect);

            let filters_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);
            
            let input_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);
            
            let filters = List::new(app_lock.admin.filters.clone())
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .block(filters_block);

            f.render_stateful_widget(filters, popup_chunks[0], &mut app_lock.admin.filter_list_state);
            f.render_widget(input_block, popup_chunks[1]);

            match app_lock.admin.active_filter {
                Some(Filter::Username) | Some(Filter::Name) |
                Some(Filter::Ci) | Some(Filter::AccNum) |
                Some(Filter::Balance) => {
                    let input_rect = centered_rect(
                        percent_x(f, 2.0),
                        percent_y(f, 0.5),
                        popup_chunks[1]);
                    
                    let input_block = Block::default().borders(Borders::BOTTOM).border_type(BorderType::Thick).title("Input");

                    let input = Paragraph::new(Line::from(
                        Span::raw(app_lock.input.0.value())
                    ))
                    .block(input_block);

                    f.render_widget(input, input_rect);
                }
                Some(Filter::AccType) | Some(Filter::AccStatus) => {
                    let options_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Min(3),
                            Constraint::Percentage(100),
                            Constraint::Min(3),
                        ])
                        .split(popup_chunks[1].inner(&Margin::new(6, 1)));

                    let option_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);
                    
                    let (option1_text, option2_text) = match app_lock.admin.active_filter {
                        Some(Filter::AccType) => ("Current", "Debit"),
                        Some(Filter::AccStatus) => ("Suspended", "Not suspended"),
                        _ => panic!()
                    };

                    let option1 = Paragraph::new(Line::from(
                        Span::raw(format!("{}", option1_text))
                    ))
                    .block(option_block.clone())
                    .alignment(Alignment::Center);
                    
                    let option2 = Paragraph::new(Line::from(
                        Span::raw(format!("{}", option2_text))
                    ))
                    .block(option_block)
                    .alignment(Alignment::Center);

                    f.render_widget(option1, options_chunks[0]);
                    f.render_widget(option2, options_chunks[2]);
                }
                None => {}
                _ => { todo!("filter sidescreen") }
            }
        }
        Some(Popup::AddClient) => todo!("add client popup"),
        _ => {}
    }
}