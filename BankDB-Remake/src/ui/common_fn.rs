use ratatui::{
    layout::{Layout, Direction, Rect, Constraint},
    prelude::Frame,
    widgets::Clear,
};

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
pub fn percent_x(f: &mut Frame, multiplier: f32) -> u16 {
    let result = ((((multiplier * 125.00) * 0.99_f32.powi(f.size().width as i32))) + 1.0) as u16;
    if result >= 100 { return 100; }
    else { return result; }
}

/// Calculates the height percentage for centering a rect with a constant height in a given frame.
/// Uses an exponential function with parameters a > 0 and c > 0. 
/// The multiplier param adjusts the rect size. Should be used with `centered_rect` function.
pub fn percent_y(f: &mut Frame, multiplier: f32) -> u16 {
    let result = ((multiplier * (130.00 * 0.95_f32.powi(f.size().height as i32))) + 3.0) as u16;
    if result >= 100 { return 100; }
    else { return result; }
}

pub fn clear_chunks(f: &mut Frame, chunks: &std::rc::Rc<[Rect]>) {
    for chunk in chunks.iter() {
        f.render_widget(Clear, *chunk);
    }
}