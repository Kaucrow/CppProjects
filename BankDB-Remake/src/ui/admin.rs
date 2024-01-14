use ratatui::{
    layout::{Layout, Direction, Constraint, Margin},
    prelude::{Alignment, Frame},
    style::{Color, Style, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, List, Row, Table, BorderType, Borders, Paragraph, Clear}
};
use std::sync::{Arc, Mutex};
use crate::{
    model::{
        common::{Popup, CltData, ScreenSection, Button},
        app::App
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
            percent_x(f, 2.3),
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
        Span::raw(&app_lock.help_text
    )))
    .block(help_block);

    f.render_widget(help, chunks[1]);

    let (left_fg_color, right_fg_color) =
        if let ScreenSection::Left = app_lock.active_screen_section {
            (Color::White, Color::DarkGray)
        } else {
            (Color::DarkGray, Color::White)
        };

    let admin_title = Paragraph::new(Line::from(vec![
        Span::raw(" Login: "),
        Span::styled("Admin", Style::default().fg(Color::Yellow))
    ]))
    .block(Block::default().borders(Borders::BOTTOM));

    f.render_widget(Block::default().borders(Borders::ALL).style(Style::default().fg(left_fg_color)), main_chunks[0]);
    f.render_widget(admin_title, left_chunks[0]);
    
    let actions = List::new(app_lock.admin.actions.clone())
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_stateful_widget(actions, left_chunks[1], &mut app_lock.admin.action_list_state);
    
    let client_table_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ])
        .split(main_chunks[1].inner(&Margin::new(0, 1)));

    let client_table_block = Block::default().borders(Borders::ALL).style(Style::default().fg(right_fg_color));
    
    let header =
        Row::new(vec!["Username", "Name", "C.I.", "Acc. num.",])
        .style(Style::default().fg(Color::Cyan));

    let widths = [
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(9),
        Constraint::Length(9),
    ];
    
    let rows: Vec<Row> =
        app_lock.admin.stored_clients
        .iter()
        .map(|client| {
            Row::new(vec![
                client.username.clone(),
                client.name.clone(),
                client.ci.to_string(),
                client.account_number.to_string()])
        })
        .collect();

    let client_table = Table::new(rows, widths)
        .column_spacing(3)
        .header(header.bottom_margin(1))
        .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::REVERSED));
        //.block(Block::default().borders(Borders::ALL));
    
    f.render_widget(client_table_block, main_chunks[1]);
    f.render_stateful_widget(client_table, client_table_chunks[1], &mut app_lock.admin.client_table_state);
    
    match app_lock.active_popup {
        Some(Popup::FilterClients) | Some(Popup::AddClient) => {
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
                    
            let (left_fg_color, right_fg_color) = if let ScreenSection::Left = app_lock.admin.popup_screen_section {
                (Color::White, Color::DarkGray)
            } else {
                (Color::DarkGray, Color::White)
            };

            let filters_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(left_fg_color));
            
            let input_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .style(Style::default().fg(right_fg_color));
            
            let filters = List::new(app_lock.admin.cltdata.clone())
                .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
                .block(filters_block);

            f.render_stateful_widget(filters, popup_chunks[0], &mut app_lock.admin.cltdata_list_state);
            f.render_widget(input_block, popup_chunks[1]);

            match app_lock.admin.active_cltdata {
                Some(CltData::Username) | Some(CltData::Name) |
                Some(CltData::Ci) | Some(CltData::AccNum) |
                Some(CltData::Balance) => {
                    let input_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Percentage(20),
                            Constraint::Length(3),
                            Constraint::Percentage(100),
                        ])
                        .split(popup_chunks[1].inner(&Margin::new(6, 2)));
                    
                    let input_block = Block::default().borders(Borders::ALL).title("Input");

                    let input = Paragraph::new(Line::from(
                        Span::styled(app_lock.input.0.value(), Style::default().fg(Color::Green))
                    ))
                    .block(input_block);

                    f.render_widget(input, input_chunks[1]);

                    f.set_cursor(input_chunks[1].x
                                    + app_lock.input.0.visual_cursor() as u16
                                    + 1,
                                 input_chunks[1].y + 1,
                                );
                }
                Some(CltData::AccType) | Some(CltData::AccStatus) => {
                    let options_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Min(3),
                            Constraint::Percentage(100),
                            Constraint::Min(3),
                        ])
                        .split(popup_chunks[1].inner(&Margin::new(6, 1)));

                    let option_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

                    let (option1_fg_color, option2_fg_color) = match app_lock.admin.popup_screen_section {
                        ScreenSection::Left => {
                            match app_lock.admin.button_selection {
                                Some(Button::Up) => (Color::Green, Color::DarkGray),
                                Some(Button::Down) => (Color::DarkGray, Color::Green),
                                None => (Color::DarkGray, Color::DarkGray)
                            }
                        }
                        ScreenSection::Right => {
                            match app_lock.admin.button_selection {
                                Some(Button::Up) => (Color::Green, Color::White),
                                Some(Button::Down) => (Color::White, Color::Green),
                                None => (Color::White, Color::White)
                            }
                        }
                        _ => panic!()
                    };

                    let (option1_text, option2_text) = match app_lock.admin.active_cltdata {
                        Some(CltData::AccType) => ("Current", "Debit"),
                        Some(CltData::AccStatus) => ("Suspended", "Not suspended"),
                        _ => panic!()
                    };

                    let option1 = Paragraph::new(Line::from(
                        Span::raw(format!("{}", option1_text))
                    ))
                    .style(Style::default().fg(option1_fg_color))
                    .block(option_block.clone())
                    .alignment(Alignment::Center);
                    
                    let option2 = Paragraph::new(Line::from(
                        Span::raw(format!("{}", option2_text))
                    ))
                    .style(Style::default().fg(option2_fg_color))
                    .block(option_block)
                    .alignment(Alignment::Center);

                    f.render_widget(option1, options_chunks[0]);
                    f.render_widget(option2, options_chunks[2]);
                }
                None => {}
                _ => { todo!("filter sidescreen") }
            }
        }
        Some(Popup::AddClientPsswd) => {
            let popup_rect = centered_rect(
                percent_x(f, 0.8),
                percent_y(f, 0.5), 
                f.size()
            );

            f.render_widget(Clear, popup_rect);

            let popup_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(40),
                    Constraint::Min(3),
                    Constraint::Percentage(50),
                ])
                .split(popup_rect.inner(&Margin::new(2, 0)));

            let popup_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title("Add a password");

            let input = Paragraph::new(Line::from(
                Span::raw(format!("{}", app_lock.input.0))
            ));

            f.render_widget(popup_block, popup_rect);
            f.render_widget(input, popup_chunks[1]);

            f.set_cursor(popup_chunks[1].x
                                    + app_lock.input.0.visual_cursor() as u16,
                         popup_chunks[1].y,
                        );
        }
        Some(Popup::AddClientSuccess) => {
            let popup_rect = centered_rect(
                percent_x(f, 0.6),
                percent_y(f, 0.4),
                f.size()
            );

            f.render_widget(Clear, popup_rect);

            let text = Paragraph::new(vec![
                Line::from(Span::raw("Added client")),
                Line::from(Span::raw("successfully."))
            ])
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).border_type(BorderType::Thick));

            f.render_widget(text, popup_rect);
        }
        _ => {}
    }
}