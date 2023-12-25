use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{Ferris, Dither, IndexType, Status};

pub fn ui(f: &mut Frame, ferris: &mut Ferris) {
    if f.size().height < ferris.normal.len() as u16 { return; }
    let extra_space = f.size().height - ferris.normal.len() as u16;
    let bottom_space = extra_space / 2;
    let top_space = extra_space - bottom_space;    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            std::iter::once(Constraint::Length(top_space))
            .chain(std::iter::repeat(Constraint::Min(1)).take(ferris.normal.len()))
            .chain(std::iter::once(Constraint::Length(bottom_space))),
        )
        .split(f.size());

    for line_idx in 0..ferris.normal.len() as u8 {
        let mut content = ferris.normal.get(line_idx as usize).unwrap();
        let mut style = Style::default();

        for (dither, mask_idx) in ferris.mask.iter() {
            if Status::Index(line_idx) == *mask_idx {
                match *dither {
                    Dither::Light => {
                        content = ferris.light.get(line_idx as usize).unwrap();
                        break;
                    }
                    Dither::Normal => {
                        break;
                    } 
                    _ => { unimplemented!("dither of type {:?} has no assigned action", dither) }
                }
            }
        }

        let render_line = Paragraph::new(Text::styled(
            content,
            style,
        ));

        f.render_widget(render_line, chunks[(line_idx + 1) as usize]);
    }

}