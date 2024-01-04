use crossterm::event::{
    self,
    Event as CrosstermEvent,
    KeyEventKind,
    KeyCode,
    KeyEvent,
    KeyModifiers,
};
use rust_decimal::Decimal;
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
    TimeoutType,
    InputMode,
};

#[derive(Debug)]
pub enum InputBlacklist {
    None,
    Money,
}

/// Terminal events
//#[derive(Debug)]
pub enum Event {
    Quit,
    ExitPopup,
    TryLogin,
    EnterScreen(Screen),
    KeyInput(KeyEvent, InputBlacklist),
    SwitchInput,
    NextClientAction,
    PreviousClientAction,
    SelectAction,
    Deposit,
    Withdraw,
    Transfer,
    ChangePasswd,
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

    /// Receive the next event from the handler thread
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

            // Events common to all screens.
            match key_event.code {
                KeyCode::Char('c') if key_event.modifiers == KeyModifiers::CONTROL => sender.send(Event::Quit),
                _ if app_lock.hold_popup => { sender.send(Event::ExitPopup).expect("could not send terminal event"); return; },
                _ => Ok(())
            }.expect("could not send terminal event");

            // Screen-specific events.
            match app_lock.curr_screen {
                Screen::Login => {
                    match app_lock.active_popup {
                        Some(Popup::LoginSuccessful) => {
                            if let Some(user) = &app_lock.active_user {
                                if user.name == "admin" { sender.send(Event::EnterScreen(Screen::Admin)) }
                                else { sender.send(Event::EnterScreen(Screen::Client)) }
                            } else {
                                Ok(())
                            }.expect("could not send terminal event");
                        },
                        None => {
                            match key_event.code {
                                KeyCode::Esc => sender.send(Event::Quit),
                                KeyCode::Enter => sender.send(Event::TryLogin),
                                KeyCode::Tab => sender.send(Event::SwitchInput),
                                _ => sender.send(Event::KeyInput(key_event, InputBlacklist::None)),
                            }.expect("could not send terminal event");
                        }
                        _ => { unimplemented!("popup not found in match block") }
                    }
                },
                Screen::Client => {
                    match app_lock.active_popup {
                        Some(Popup::ViewInfo) => {
                            match key_event.code {
                                KeyCode::Esc => sender.send(Event::ExitPopup),
                                _ => Ok(())
                            }.expect("could not send terminal event");
                        },
                        Some(Popup::Deposit) | Some(Popup::Withdraw) => {
                            match key_event.code {
                                KeyCode::Esc => sender.send(Event::ExitPopup),
                                KeyCode::Enter => {
                                    if let Some(Popup::Deposit) = app_lock.active_popup { sender.send(Event::Deposit) }
                                    else { sender.send(Event::Withdraw) }
                                }
                                _ => sender.send(Event::KeyInput(key_event, InputBlacklist::Money))
                            }.expect("could not send terminal event");
                        },
                        Some(Popup::Transfer) => {
                            match key_event.code {
                                KeyCode::Esc => sender.send(Event::ExitPopup),
                                KeyCode::Tab => sender.send(Event::SwitchInput),
                                KeyCode::Enter => {
                                    sender.send(Event::Transfer)
                                },
                                _ => {
                                    if let InputMode::Editing(field) = app_lock.input_mode {
                                        if field == 0 { sender.send(Event::KeyInput(key_event, InputBlacklist::Money)) }
                                        else { sender.send(Event::KeyInput(key_event, InputBlacklist::None)) }
                                    } else { Ok(()) }
                                }
                            }.expect("could not send terminal event");
                        },
                        Some(Popup::ChangePsswd) => {
                            match key_event.code {
                                KeyCode::Esc => sender.send(Event::ExitPopup),
                                KeyCode::Tab => sender.send(Event::SwitchInput),
                                KeyCode::Enter => sender.send(Event::ChangePasswd),
                                _ => sender.send(Event::KeyInput(key_event, InputBlacklist::None))
                            }.expect("could not send terminal event");
                        },
                        None => {
                            match key_event.code {
                                KeyCode::Esc => sender.send(Event::EnterScreen(Screen::Login)),
                                KeyCode::Char('k') | KeyCode::Up => sender.send(Event::PreviousClientAction),
                                KeyCode::Char('j') | KeyCode::Down => sender.send(Event::NextClientAction),
                                KeyCode::Enter => sender.send(Event::SelectAction),
                                _ => { Ok(()) }
                            }.expect("could not send terminal event");
                        }
                        _ => { unimplemented!("popup not found in match block") }
                    }
                },
                Screen::Admin => {
                    match app_lock.active_popup {
                        None => {
                            match key_event.code {
                                KeyCode::Esc => sender.send(Event::EnterScreen(Screen::Login)),
                                _ => Ok(())
                            }.expect("could not send terminal event")
                        }
                        _ => todo!("popups on admin screen")
                    }
                }
                _ => { unimplemented!("screen not found in match block") }
            }
        },
        CrosstermEvent::Resize(_, _) => {
            let mut app_lock = app.lock().unwrap();
            if !app_lock.timeout.contains_key(&TimeoutType::Resize) {
                app_lock.add_timeout(1, 250, TimeoutType::Resize);
                sender.send(Event::Resize)
            } else {
                Ok(())
            }
        }.expect("could not send terminal event"),
        _ => {}
    }
}