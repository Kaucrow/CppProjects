use crossterm::event::{
    self,
    Event as CrosstermEvent,
    KeyEventKind,
    KeyCode,
    KeyEvent,
    MouseEvent, KeyModifiers,
};

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant}
};

use anyhow::{ Result, Error };

use crate::model::{ App, Screen };

/// Terminal events
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Quit,
    TryLogin,
    Key(KeyEvent),
    SwitchInput,
    Resize,
}

#[derive(Debug)]
pub struct EventHandler {
    // Event sender channel
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    // Event receiver channel
    receiver: mpsc::Receiver<Event>,
    // Event handler thread
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>
}

impl EventHandler {
    // Constructs a new instance of [`EventHandler`]
    pub fn new(tick_rate: u64, app_arc: &Arc<Mutex<App>>) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let app_arc = Arc::clone(&app_arc);
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for event") {
                        event_act(event::read().expect("unable to read event"), &sender, &app_arc);
                    }

                    if last_tick.elapsed() >= tick_rate {
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receiver the next event from the handler thread
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}

fn event_act(event: CrosstermEvent, sender: &mpsc::Sender<Event>, app: &Arc<Mutex<App>>) {
    match event {
        CrosstermEvent::Key(key_event) => {
            if key_event.kind == KeyEventKind::Release { return; }
            match app.lock().unwrap().curr_screen {
                Screen::Login => {
                    match key_event.code {
                        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => { sender.send(Event::Quit) },
                        KeyCode::Enter => { sender.send(Event::TryLogin) }
                        KeyCode::Tab => { sender.send(Event::SwitchInput) }
                        _ => { sender.send(Event::Key(key_event)) }
                    }.expect("could not send terminal event");
                }
                _ => {}
            }
        },
        CrosstermEvent::Resize(x, y) => {
            sender.send( Event::Resize )
        }.expect("could not send terminal event"),
        _ => {}
    }
}