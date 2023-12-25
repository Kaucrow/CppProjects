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

use std::{time::{Instant, Duration}, collections::BTreeMap};

use std::io::{self, BufRead, BufReader};
use std::fs::File;

use anyhow::Result;

use old_tv::app::{ Ferris, Dither, IndexType, Status };

use old_tv::ui::ui;

fn main() -> Result<()>{
    let data_path = {
        let mut path = String::from(std::env::current_exe().unwrap().to_string_lossy());
        path = path[..=path.find("old_tv").expect("could not find `old_tv` folder") + ("old_tv").len()].to_string();
        if cfg!(windows){
            path.push_str("src\\data\\");
        } else {
            path.push_str("src/data/");
        }
        path
    };

    let mut ferris = Ferris::new([Dither::Light, Dither::Light, Dither::Normal, Dither::Normal, Dither::Light, Dither::Light, Dither::Normal, Dither::Light]);

    fn get_ferris_lines<'a>(filepath: &str, ferris: & mut Ferris, light_variant: bool) -> Result<()> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line_unwrapped = line?;
            if light_variant {
                ferris.light.push(line_unwrapped.clone());
            } else {
                ferris.normal.push(line_unwrapped.clone());
            }
            println!("{}", &line_unwrapped);
        }

        Ok(())
    }
    get_ferris_lines(&(data_path.clone() + "lain.txt"), &mut ferris, false).expect("Error reading and writing");
    get_ferris_lines(&(data_path.clone() + "lain_light.txt"), &mut ferris, true).expect("Error reading and writing");

    if ferris.normal.len() != ferris.light.len() { return Err(anyhow::anyhow!("Normal and light variants must have the same number of lines")) };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;

    let res = run_app(&mut terminal, &mut ferris);

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
    ferris: &mut Ferris,

) -> Result<()> {
    let mut last_update = Instant::now();
    let update_rate = Duration::from_millis(100);
    let mut should_update = true;

    loop {
        if Instant::now() - last_update >= update_rate {
            should_update = true;
            last_update = Instant::now();
        }

        if should_update {
            for (_, index) in ferris.mask.iter_mut() {
                match *index {
                    Status::Ready => {
                        *index = Status::Index((ferris.normal.len() - 1) as IndexType);
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

            should_update = false;
            //println!("{:?}", ferris.mask);
        }


        if ferris.mask.iter().all(|(_, index)| index == &Status::Done) {
            ferris.mask.iter_mut().for_each(|(_, index)| *index = Status::Ready);
        }
        
        terminal.draw(|f| ui(f, ferris))?;

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
