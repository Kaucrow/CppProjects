use ratatui::{
    layout::{Layout, Direction, Rect, Constraint, Margin},
    prelude::{Alignment, Frame},
    style::{Color, Style, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, List, BorderType, Borders, Paragraph, Clear}
};
use std::sync::{Arc, Mutex};
use crate::model::{app::{
    App,
    Popup,
    Screen,
    InputMode,
    TimeoutType, Filter
}, client};

pub fn render(app: &mut Arc<Mutex<App>>, f: &mut Frame) {
    let mut app_lock = app.lock().unwrap();

    match app_lock.curr_screen {
        Screen::Login => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(centered_rect(
                    percent_x(f, 1.0),
                    percent_y(f, 1.0),
                    f.size()));

            if app_lock.should_clear_screen {
                clear_screen(f, &chunks);
                app_lock.should_clear_screen = false;
            }

            let title_block = Block::default();

            let title = Paragraph::new(Text::from(
                "BankDB"
            ))
            .block(title_block)
            .alignment(Alignment::Center);

            f.render_widget(title, chunks[0]);

            let width = chunks[0].width.max(3) - 3;
            let name_scroll = app_lock.input.0.visual_scroll(width as usize - "* Username: ".len());
            let password_scroll = app_lock.input.1.visual_scroll(width as usize - "* Password: ".len());
            let mut name_style = Style::default();
            let mut password_style = Style::default();
            
            if let InputMode::Editing(field) = app_lock.input_mode {
                if field == 0 {
                    password_style = password_style.fg(Color::DarkGray);
                    f.set_cursor(chunks[1].x
                                    + ((app_lock.input.0.visual_cursor()).max(name_scroll) - name_scroll) as u16
                                    + "* Username: ".len() as u16
                                    + 1,
                                chunks[1].y + 1,
                                );
                } else {
                    name_style = password_style.fg(Color::DarkGray);
                    f.set_cursor(chunks[2].x
                                    + ((app_lock.input.1.visual_cursor()).max(password_scroll) - password_scroll) as u16
                                    + "* Password: ".len() as u16
                                    + 1,
                                chunks[2].y + 1,
                                );
                }
            }
            
            let name_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(name_style);

            let input = Paragraph::new(Text::from(Line::from(vec![
                Span::styled("* Username: ", Style::default().fg(Color::Yellow)),
                Span::styled(app_lock.input.0.value(), name_style)
            ])))
            .block(name_block)
            .scroll((0, name_scroll as u16));

            f.render_widget(input, chunks[1]);
            
            let password_block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(password_style);

            let input = Paragraph::new(Text::from(Line::from(vec![
                Span::styled("* Password: ", Style::default().fg(Color::Yellow)),
                Span::styled(app_lock.input.1.value().to_string(), password_style)
            ])))
            .block(password_block)
            .scroll((0, password_scroll as u16));

            f.render_widget(input, chunks[2]);
            
            let help_text = {
                if app_lock.failed_logins == 3 {
                    Text::from(format!("Login failed - Try again in: {}", app_lock.timeout.get(&TimeoutType::Login).unwrap().counter))
                }
                else if app_lock.failed_logins > 0 {
                    Text::from("Login failed")
                } else {
                    Text::from("Press `Alt` to switch input")
                }
            };
            let help_block = Block::default();
            let help = Paragraph::new(help_text).block(help_block);
            f.render_widget(help, chunks[3]);
            
            if let Some(popup) = &app_lock.active_popup {
                match popup {
                    Popup::LoginSuccessful => {
                        let popup_rect = centered_rect(15, (3.0 / f.size().height as f32 * 100.0 + 1.0) as u16, f.size());

                        f.render_widget(Clear, popup_rect);

                        let login_successful_block = Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Thick);

                        let login_successful_popup = Paragraph::new(Text::from(
                            "Login successful."
                        ))
                        .alignment(Alignment::Center)
                        .block(login_successful_block);

                        f.render_widget(login_successful_popup, popup_rect);

                        f.set_cursor(f.size().width, f.size().height);
                    }
                    _ => { unimplemented!() }
                }
            }
        },
        Screen::Client => {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Percentage(100),
                    Constraint::Min(2)
                ])
                .split(centered_rect(
                    percent_x(f, 2.0),
                    percent_y(f, 1.5),
                    f.size()));
            
            if app_lock.should_clear_screen {
                clear_screen(f, &chunks);
                app_lock.should_clear_screen = false;
            }

            let header_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])       
                .split(chunks[0]);
            
            let header_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded); 

            let header_login = Paragraph::new(Text::from(
                format!("\n  Login: {}", app_lock.client.active.as_ref().unwrap().name
            )));
            
            let header_balance = Paragraph::new(Text::from(
                format!("\nBalance: {}$  ", app_lock.client.active.as_ref().unwrap().balance
            ))).alignment(Alignment::Right);

            f.render_widget(header_login, header_chunks[0]);
            f.render_widget(header_balance, header_chunks[1]);
            f.render_widget(header_block, chunks[0]); 

            let list_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(5),
                    Constraint::Percentage(90),
                    Constraint::Percentage(5)
                ])
                .split(chunks[1]);

            let actions = List::new(app_lock.client.actions.clone()).highlight_style(Style::default().add_modifier(Modifier::REVERSED));
            
            f.render_stateful_widget(actions, list_chunks[1], &mut app_lock.client.action_list_state);

            let help_text = Paragraph::new(Text::from(
                format!("{}", app_lock.help_text)
            ))
            .block(Block::default().borders(Borders::TOP));

            f.render_widget(help_text, chunks[2]);

            match app_lock.active_popup {
                Some(Popup::ViewInfo) => {
                    let popup_rect = centered_rect(
                        percent_x(f, 1.0),
                        percent_y(f, 1.0),
                        f.size()
                    );

                    f.render_widget(Clear, popup_rect);
                    
                    let client_info_block = Block::default().borders(Borders::ALL).border_type(BorderType::QuadrantOutside);

                    let active_user = app_lock.client.active.as_ref().unwrap();
                    let client_info = Paragraph::new(vec![
                        Line::from(Span::raw("Client Information")),
                        Line::default(),
                        Line::from(Span::raw(format!("Full name: {}", active_user.name))),
                        Line::from(Span::raw(format!("C.I.: {}", active_user.ci))),
                        Line::from(Span::raw(format!("Account Num.: {}", active_user.account_number))),
                        Line::from(Span::raw(format!("Account Type: {:?}", active_user.account_type))),
                        Line::from(Span::raw(format!("Balance: {}$", active_user.balance)))
                    ])
                    .alignment(Alignment::Center)
                    .block(client_info_block);

                    f.render_widget(client_info, popup_rect);
                },
                Some(Popup::Deposit) | Some(Popup::Withdraw) => {
                    let popup_rect = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Length(3),
                            Constraint::Percentage(100)
                        ]).split(centered_rect(
                        percent_x(f, 1.0),
                        percent_y(f, 0.4),
                        f.size())
                    );

                    f.render_widget(Clear, popup_rect[0]);

                    let title = {
                        if let Some(Popup::Deposit) = app_lock.active_popup { "Deposit amount" }
                        else { "Withdraw amount" }
                    };

                    let deposit_block = Block::default().borders(Borders::ALL).border_type(BorderType::Thick).title(title);

                    let deposit = Paragraph::new(Line::from(vec![
                        Span::raw(" "),
                        Span::raw(app_lock.input.0.value()),
                    ]))
                    .block(deposit_block)
                    .alignment(Alignment::Left);

                    f.render_widget(deposit, popup_rect[0]);

                    f.set_cursor(
                        popup_rect[0].x
                        + app_lock.input.0.visual_cursor() as u16
                        + 2,
                        popup_rect[0].y + 1);
                }
                Some(Popup::Transfer) | Some(Popup::ChangePsswd) => {
                    let popup_rect = centered_rect(
                        percent_x(f, 1.0),
                        percent_y(f, 0.9),
                        f.size()
                    );

                    let popup_chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([
                            Constraint::Min(3),
                            Constraint::Length(1),
                            Constraint::Min(3),
                        ])
                        .split(popup_rect.inner(&Margin::new(1, 1)));
                    
                    let popup_block_title: &str;
                    let upper_block_title: &str;
                    let lower_block_title: &str;
                    
                    if let Some(Popup::Transfer) = app_lock.active_popup {
                        popup_block_title = "Transfer";
                        upper_block_title = "Amount";
                        lower_block_title = "Beneficiary";
                    } else {
                        popup_block_title = "Change Password";
                        upper_block_title = "Old Password";
                        lower_block_title = "New Password";
                    }

                    let popup_block = Block::default().borders(Borders::ALL).border_type(BorderType::Thick).title(popup_block_title);
                    let amount_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(upper_block_title);
                    let beneficiary_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(lower_block_title);

                    let amount = Paragraph::new(Line::from(vec![
                        Span::raw(" "),
                        Span::raw(app_lock.input.0.value())
                    ]))
                    .block(amount_block);
                    
                    let beneficiary = Paragraph::new(Line::from(vec![
                        Span::raw(" "),
                        Span::raw(app_lock.input.1.value())
                    ]))
                    .block(beneficiary_block);
        
                    if let InputMode::Editing(field) = app_lock.input_mode {
                        if field == 0 {
                            f.set_cursor(popup_chunks[0].x
                                            + app_lock.input.0.visual_cursor() as u16
                                            + 2,
                                        popup_chunks[0].y + 1,
                                        );
                        } else {
                            f.set_cursor(popup_chunks[2].x
                                            + app_lock.input.1.visual_cursor() as u16
                                            + 2,
                                        popup_chunks[2].y + 1,
                                        );
                        }
                    }
                    
                    f.render_widget(Clear, popup_rect);
                    f.render_widget(popup_block, popup_rect);
                    f.render_widget(amount, popup_chunks[0]);
                    f.render_widget(beneficiary, popup_chunks[2]);
                }
                _ => {}
            }
        },
        Screen::Admin => {
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
                            let options_rect = centered_rect(
                                percent_x(f, 2.0),
                                percent_y(f, 1.0),
                                popup_chunks[1]);

                            let options_chunks = Layout::default()
                                .direction(Direction::Vertical)
                                .constraints([
                                    Constraint::Min(3),
                                    Constraint::Percentage(100),
                                    Constraint::Min(3),
                                ])
                                .split(options_rect);

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

/// Calculates the width percentage for centering a rect with a constant width in a given frame.
/// Uses an exponential function with parameters a > 0 and c > 0. 
/// The multiplier param adjusts the rect size. Should be used with `centered_rect` function.
fn percent_x(f: &mut Frame, multiplier: f32) -> u16 {
    let result = ((((multiplier * 125.00) * 0.99_f32.powi(f.size().width as i32))) + 1.0) as u16;
    if result >= 100 { return 100; }
    else { return result; }
}

/// Calculates the height percentage for centering a rect with a constant height in a given frame.
/// Uses an exponential function with parameters a > 0 and c > 0. 
/// The multiplier param adjusts the rect size. Should be used with `centered_rect` function.
fn percent_y(f: &mut Frame, multiplier: f32) -> u16 {
    let result = ((multiplier * (130.00 * 0.95_f32.powi(f.size().height as i32))) + 3.0) as u16;
    if result >= 100 { return 100; }
    else { return result; }
}

fn clear_screen(f: &mut Frame, chunks: &std::rc::Rc<[Rect]>) {
    for chunk in chunks.iter() {
        f.render_widget(Clear, *chunk);
    }
}