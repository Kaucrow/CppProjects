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
use crate::model::{
    common::{
        Popup, Screen, TimeoutType, ListType, TableType, InputMode,
        ScreenSection, ScreenSectionType, Filter
    },
    app::App,
};

#[derive(Debug)]
pub enum InputBlacklist {
    None,
    Money,
    Alphabetic,
    Numeric,
}

/// Terminal events
#[derive(Debug)]
pub enum Event {
    Quit,
    Cleanup,
    TryLogin,
    EnterScreen(Screen),
    KeyInput(KeyEvent, InputBlacklist),
    SwitchInput,
    SwitchScreenSection(ScreenSectionType),
    NextListItem(ListType),
    PreviousListItem(ListType),
    SelectAction(ListType),
    NextTableItem(TableType),
    PreviousTableItem(TableType),
    Deposit,
    Withdraw,
    Transfer,
    ChangePasswd,
    EditFilter,
    RegisterFilter,
    ApplyFilters,
    SwitchButton,
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
                _ if app_lock.hold_popup => { sender.send(Event::Cleanup).expect("could not send terminal event"); return; },
                _ => Ok(())
            }.expect("could not send terminal event");

            // Screen-specific events.
            match app_lock.active_screen {
                Screen::Login => {
                    match app_lock.active_popup {
                        Some(Popup::LoginSuccessful) => {
                            if let Some(user) = &app_lock.client.active {
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
                                KeyCode::Esc => sender.send(Event::Cleanup),
                                _ => Ok(())
                            }.expect("could not send terminal event");
                        },
                        Some(Popup::Deposit) | Some(Popup::Withdraw) => {
                            match key_event.code {
                                KeyCode::Esc => sender.send(Event::Cleanup),
                                KeyCode::Enter => {
                                    if let Some(Popup::Deposit) = app_lock.active_popup { sender.send(Event::Deposit) }
                                    else { sender.send(Event::Withdraw) }
                                }
                                _ => sender.send(Event::KeyInput(key_event, InputBlacklist::Money))
                            }.expect("could not send terminal event");
                        },
                        Some(Popup::Transfer) => {
                            match key_event.code {
                                KeyCode::Esc => sender.send(Event::Cleanup),
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
                                KeyCode::Esc => sender.send(Event::Cleanup),
                                KeyCode::Tab => sender.send(Event::SwitchInput),
                                KeyCode::Enter => sender.send(Event::ChangePasswd),
                                _ => sender.send(Event::KeyInput(key_event, InputBlacklist::None))
                            }.expect("could not send terminal event");
                        },
                        None => {
                            match key_event.code {
                                KeyCode::Esc => {
                                    sender.send(Event::Cleanup).expect("could not send terminal event");
                                    sender.send(Event::EnterScreen(Screen::Login))
                                }
                                KeyCode::Char('k') | KeyCode::Up => sender.send(Event::PreviousListItem(ListType::ClientAction)),
                                KeyCode::Char('j') | KeyCode::Down => sender.send(Event::NextListItem(ListType::ClientAction)),
                                KeyCode::Enter => sender.send(Event::SelectAction(ListType::ClientAction)),
                                _ => Ok(())
                            }.expect("could not send terminal event");
                        }
                        _ => { unimplemented!("popup not found in match block") }
                    }
                },
                Screen::Admin => {
                    match app_lock.active_popup {
                        Some(Popup::FilterClients) => {
                            match app_lock.admin.popup_screen_section {
                                ScreenSection::Left => {
                                    match key_event.code {
                                        KeyCode::Esc => sender.send(Event::Cleanup),
                                        KeyCode::Enter => {
                                            sender.send(Event::SwitchScreenSection(ScreenSectionType::AdminFilters)).expect("could not send terminal event");
                                            sender.send(Event::EditFilter)
                                        }
                                        KeyCode::Char('a') => {
                                            sender.send(Event::ApplyFilters).expect("could not send terminal event");
                                            sender.send(Event::Cleanup)
                                        }
                                        KeyCode::Char('k') | KeyCode::Up => sender.send(Event::PreviousListItem(ListType::ClientFilters)),
                                        KeyCode::Char('j') | KeyCode::Down => sender.send(Event::NextListItem(ListType::ClientFilters)),
                                        _ => Ok(())
                                    }.expect("could not send terminal event");
                                }
                                ScreenSection::Right => {
                                    match key_event.code {
                                        KeyCode::Esc => {
                                            sender.send(Event::SwitchScreenSection(ScreenSectionType::AdminFilters))
                                        }
                                        KeyCode::Enter => {
                                            sender.send(Event::SwitchScreenSection(ScreenSectionType::AdminFilters)).expect("could not send terminal event");
                                            sender.send(Event::RegisterFilter)
                                        }
                                        _ => Ok(())
                                    }.expect("could not send terminal event");

                                    match app_lock.admin.active_filter {
                                        Some(Filter::Username) => {
                                            sender.send(Event::KeyInput(key_event, InputBlacklist::None))
                                        }
                                        Some(Filter::Name) => {
                                            sender.send(Event::KeyInput(key_event, InputBlacklist::Alphabetic))
                                        }
                                        Some(Filter::Ci) | Some(Filter::AccNum) => {
                                            sender.send(Event::KeyInput(key_event, InputBlacklist::Numeric))
                                        }
                                        Some(Filter::Balance) => {
                                            sender.send(Event::KeyInput(key_event, InputBlacklist::Money))
                                        }
                                        Some(Filter::AccStatus) | Some(Filter::AccType) => {
                                            match key_event.code {
                                                KeyCode::Tab => {
                                                    sender.send(Event::SwitchButton)
                                                }
                                                _ => Ok(())
                                            }
                                        }
                                        _ => todo!("filter sidescreen events")
                                    }.expect("could not send terminal event");
                                }
                                _ => {}
                            }
                        }
                        None => {
                            match key_event.code {
                                KeyCode::Esc => {
                                    sender.send(Event::Cleanup).expect("could not send terminal event");
                                    sender.send(Event::EnterScreen(Screen::Login))
                                }
                                KeyCode::Tab => sender.send(Event::SwitchScreenSection(ScreenSectionType::AdminMain)),
                                _ => Ok(())
                            }.expect("could not send terminal event");
                            match app_lock.active_screen_section {
                                ScreenSection::Left => {
                                    match key_event.code {
                                        KeyCode::Char('k') | KeyCode::Up => sender.send(Event::PreviousListItem(ListType::AdminAction)),
                                        KeyCode::Char('j') | KeyCode::Down => sender.send(Event::NextListItem(ListType::AdminAction)),
                                        KeyCode::Enter => sender.send(Event::SelectAction(ListType::AdminAction)),
                                        _ => Ok(())
                                    }.expect("could not send terminal event");
                                }
                                ScreenSection::Right => {
                                    match key_event.code {
                                        KeyCode::Char('k') | KeyCode::Up => sender.send(Event::PreviousTableItem(TableType::Clients)),
                                        KeyCode::Char('j') | KeyCode::Down => sender.send(Event::NextTableItem(TableType::Clients)),
                                        _ => Ok(())
                                    }.expect("could not send terminal event");
                                },
                                _ => {}
                            }
                        }
                        _ => todo!("popups on admin screen")
                    }
                }
                _ => unimplemented!("screen not found in match block")
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