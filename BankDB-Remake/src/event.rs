use crossterm::event::{
    self,
    Event as CrosstermEvent,
    KeyEventKind,
    KeyCode,
    KeyEvent,
    KeyModifiers,
};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant}
};
use anyhow::Result;
use crate::model::app::{
    App,
    Popup,
    Screen,
    TimeoutType
};

/// Terminal events
#[derive(Debug)]
pub enum Event {
    Quit,
    TryLogin,
    EnterAdminScreen,
    EnterClientScreen,
    Key(KeyEvent),
    SwitchInput,
    Resize,
    TimeoutStep(TimeoutType),
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
    pub fn new(tick_step: u16, app_arc: &Arc<Mutex<App>>) -> Self {
        let tick_rate = Duration::from_millis(tick_step as u64);
        let (sender, receiver) = mpsc::channel();
        let app_arc = Arc::clone(&app_arc);
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now(); 
                loop {
                    if event::poll(Duration::from_millis(100)).unwrap() {
                        event_act(event::read().expect("unable to read event"), &sender, &app_arc);
                    }
                    
                    if last_tick.elapsed() >= tick_rate {
                        last_tick = Instant::now();
                        for (timeout_type, timer) in &app_arc.lock().unwrap().timeout {
                            if timer.last_update.elapsed() > timer.tick_rate {
                                sender.send(Event::TimeoutStep(*timeout_type)).expect("could not send terminal event");
                            }
                        }
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
            let app_lock = app.lock().unwrap();
            match app_lock.curr_screen {
                Screen::Login => {
                    match key_event.code {
                        KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => { sender.send(Event::Quit) },
                        KeyCode::Enter => {
                            if let (Some(Popup::LoginSuccessful), Some(user)) = (&app_lock.active_popup, &app_lock.active_user) {
                                if user.name == "admin" { sender.send(Event::EnterAdminScreen) }
                                else { sender.send(Event::EnterClientScreen) }
                            } else {
                                sender.send(Event::TryLogin)
                            }
                        }
                        KeyCode::Tab => { sender.send(Event::SwitchInput) }
                        _ => { sender.send(Event::Key(key_event)) }
                    }.expect("could not send terminal event");
                }
                _ => {}
            }
        },
        CrosstermEvent::Resize(_, _) => {
            let mut app_lock = app.lock().unwrap();
            if !app_lock.timeout.contains_key(&TimeoutType::Resize) {
                app_lock.add_timeout(1, 250, TimeoutType::Resize);
                sender.send( Event::Resize )
            } else {
                Ok(())
            }
        }.expect("could not send terminal event"),
        _ => {}
    }
}