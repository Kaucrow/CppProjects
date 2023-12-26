use ratatui::{
    backend::{
        Backend,
        CrosstermBackend,
    },
    Terminal,
};
use crossterm::{
    event::{
        self,
        EnableMouseCapture,
        DisableMouseCapture,
        Event,
        KeyEventKind,
        KeyCode,
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::{time::{Instant, Duration}};
use rand::rngs::OsRng;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use anyhow::Result;
use old_tv::app::{ Image, Dither, IndexType, Status };
use old_tv::ui::ui;

enum ImageVariant {
    Normal,
    Light,
    Shift,
    LightShift
}

fn main() -> Result<()>{
    let data_path = {
        let mut path = String::from(std::env::current_exe().unwrap().to_string_lossy());
        path = path[..=path.find("old_tv").expect("could not find `old_tv` folder") + ("old_tv").len()].to_string();
        if cfg!(windows){
            path.push_str("data\\");
        } else {
            path.push_str("data/");
        }
        path
    };

    let mut image = Image::new([Dither::Light, Dither::Light, Dither::Normal, Dither::Normal, Dither::Light, Dither::Light, Dither::Normal, Dither::Light]);

    fn get_image_lines<'a>(filepath: &str, image: & mut Image, variant: ImageVariant) -> Result<()> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        //let mut sinx_count = 0;

        for line in reader.lines() {
            let line_unwrapped = line?;
            match variant {
                ImageVariant::Normal => image.normal.push(line_unwrapped),
                ImageVariant::Light => image.light.push(line_unwrapped),
                ImageVariant::Shift => image.shift.push(line_unwrapped),
                ImageVariant::LightShift => image.light_shift.push(line_unwrapped),
            }
        }

        Ok(())
    }
    get_image_lines(&(data_path.clone() + "lain.txt"), &mut image, ImageVariant::Normal).expect("Error reading and writing");
    get_image_lines(&(data_path.clone() + "lain_light.txt"), &mut image, ImageVariant::Light).expect("Error reading and writing");
    get_image_lines(&(data_path.clone() + "lain_shift.txt"), &mut image, ImageVariant::Shift).expect("Error reading and writing");
    get_image_lines(&(data_path.clone() + "lain_light_shift.txt"), &mut image, ImageVariant::LightShift).expect("Error reading and writing");

    if image.normal.len() != image.light.len() { return Err(anyhow::anyhow!("Normal and light variants must have the same number of lines")) };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;

    let res = run_app(&mut terminal, &mut image);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    image: &mut Image,

) -> Result<()> {
    let mut last_update = Instant::now();
    let update_rate = Duration::from_millis(100);
    let mut should_update = true;
    
    let rng = {
        if image.mask.is_empty() {
            None
        } else {
            Some(OsRng)
        }
    };

    let mut sinx: u32 = 0;

    loop {
        if Instant::now() - last_update >= update_rate {
            should_update = true;
            last_update = Instant::now();
        }

        if should_update {
            for (_, index) in image.mask.iter_mut() {
                match *index {
                    Status::Ready => {
                        *index = Status::Index((image.normal.len() - 1) as IndexType);
                        break;
                    },
                    Status::Index(ref mut value) => {
                        if *value > 0 {
                            *value -= 1;
                        } else {
                            *index = Status::Done;
                        }
                    },
                    _ => {}
                }
            }

            image.wave_offset.clear();
            for i in 0..(image.normal.len() as u32) {
                image.wave_offset.push((5.0 * (((sinx + i) as f32 * 0.25).sin())) as i8);
            }
            sinx += 1;

            //println!("{:?}", image.wave_offset);

            should_update = false;
            //println!("{:?}", image.mask);
        }

        if image.mask.iter().all(|(_, index)| index == &Status::Done) {
            image.mask.iter_mut().for_each(|(_, index)| *index = Status::Ready);
        }
        
        terminal.draw(|f| ui(f, image))?;

        /*if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Char('q') => {
                    return Ok(());
                }
                _ => {}
            }
        }*/
    }
    Ok(())
}
