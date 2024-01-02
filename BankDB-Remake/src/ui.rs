use ratatui::{
    layout::{Layout, Direction, Rect, Constraint},
    prelude::{Alignment, Frame},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Clear}
};
use std::sync::{Arc, Mutex};
use crate::model::app::{
    App,
    Popup,
    Screen,
    InputMode,
    TimeoutType
};

pub fn render(app: &mut Arc<Mutex<App>>, f: &mut Frame) {
    let app_lock = app.lock().unwrap();

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
                Span::styled(app_lock.input.0.value().to_string(), name_style)
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
                    Constraint::Min(1),
                    Constraint::Percentage(100),
                    Constraint::Min(1)
                ])
                .split(centered_rect(
                    percent_x(f, 2.0),
                    percent_y(f, 1.0),
                    f.size()));
            
            if app_lock.should_clear_screen {
                clear_screen(f, &chunks);
            }

            let header_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])       
                .split(chunks[0]);
            
            let header_name = Paragraph::new(Text::from(
                format!("Login: {}", app_lock.active_user.as_ref().unwrap().name
            )));

            f.render_widget(header_name, header_chunks[0]);
            
            let balance_name = Paragraph::new(Text::from(
                format!("Balance: {}", app_lock.active_user.as_ref().unwrap().balance
            ))).alignment(Alignment::Right);
            
            f.render_widget(balance_name, header_chunks[1]);

            let test = Paragraph::new(Text::from("Hello world, some text here"));
            f.render_widget(test.clone(), chunks[1]);
            f.render_widget(test, chunks[2]);
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
    let result = ((multiplier * (125.00 * 0.99_f32.powi(f.size().width as i32))) + 1.0) as u16;
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