use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{Image, Dither, IndexType, Status};

pub fn ui(f: &mut Frame, image: &mut Image) {
    if f.size().height < image.normal.len() as u16 { return; }
    let extra_space = f.size().height - image.normal.len() as u16;
    let bottom_space = extra_space / 2;
    let top_space = extra_space - bottom_space;    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            std::iter::once(Constraint::Length(top_space))
            .chain(std::iter::repeat(Constraint::Min(1)).take(image.normal.len()))
            .chain(std::iter::once(Constraint::Length(bottom_space))),
        )
        .split(f.size());

    for line_idx in 0..image.normal.len() as u8 {
        let center = 80;
        let offset;
        //let mut content = format!("{:>center$}", image.normal.get(line_idx as usize).unwrap());
        let mut content: Option<String> = None;
        let mut style = Style::default();

        /*for val in image.wave_offset.iter() {
            offset = *val / 2;
            if val % 2 == 0 {
                content = format!("{:>pos$}", image.normal.get(line_idx as usize).unwrap(), pos = center + (offset as usize));
            } else {
                content = format!("{:>pos$}", )
            }
        }*/
        let wave_offset = image.wave_offset.get(line_idx as usize).unwrap();
        offset = *wave_offset / 2;
        let pos = center + offset;

        for (dither, mask_idx) in image.mask.iter() {
            if Status::Index(line_idx) == *mask_idx {
                match *dither {
                    Dither::Light => {
                        if *wave_offset % 2 == 0 {
                            content = Some(format!("{:>pos$}", image.light.get(line_idx as usize).unwrap(), pos = pos as usize));
                        } else {
                            content = Some(format!("{:>pos$}", image.light_shift.get(line_idx as usize).unwrap(), pos = pos as usize));
                        }
                        break;
                    }
                    Dither::Normal => {
                        /*if *wave_offset % 2 == 0 {
                            content = Some(format!("{:>pos$}", image.normal.get(line_idx as usize).unwrap(), pos = pos as usize));
                        } else {
                            content = Some(format!("{:>pos$}", image.shift.get(line_idx as usize).unwrap(), pos = pos as usize));
                        }*/
                        break;
                    }
                    _ => { unimplemented!("dither of type {:?} has no assigned action", dither) }
                }
            }
        }

        if content == None {
            if *wave_offset % 2 == 0 {
                content = Some(format!("{:>pos$}", image.normal.get(line_idx as usize).unwrap(), pos = pos as usize));
            }
            else {
                content = Some(format!("{:>pos$}", image.shift.get(line_idx as usize).unwrap(), pos = pos as usize));
            }
        }

        //print!("{}, ", *wave_offset);

        let render_line = Paragraph::new(Text::styled(
            content.unwrap(),
            style,
        ));

        f.render_widget(render_line, chunks[(line_idx + 1) as usize]);
    }
    //println!();
}