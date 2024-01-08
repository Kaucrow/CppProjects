use tui_input::Input;
use std::time::{Instant, Duration};
use std::collections::HashMap;
use crate::model::{
    common::{InputFields, InputMode, TimeoutType, Timer, Screen, ScreenSection, Popup},
    admin::AdminData,
    client::ClientData
};

pub struct App {
    pub input: InputFields,
    pub input_mode: InputMode,
    pub failed_logins: u8,
    pub client: ClientData,
    pub admin: AdminData,
    pub help_text: &'static str,
    pub timeout: HashMap<TimeoutType, Timer>,
    pub curr_screen: Screen,
    pub curr_screen_section: ScreenSection,
    pub active_popup: Option<Popup>,
    pub hold_popup: bool,
    pub should_clear_screen: bool,
    pub should_quit: bool,
}

impl std::default::Default for App {
    fn default() -> Self {
        App {
            input: InputFields(Input::default(), Input::default()),
            input_mode: InputMode::Normal,
            failed_logins: 0,
            client: ClientData::default(),
            admin: AdminData::default(),
            help_text: "Choose an action to perform. Press Esc to go back.",
            timeout: HashMap::new(),
            curr_screen: Screen::Login,
            curr_screen_section: ScreenSection::Main,
            active_popup: None,
            hold_popup: false,
            should_clear_screen: false,
            should_quit: false,
        }
    }
}

impl App {
    pub fn enter_screen(&mut self, screen: &Screen) {
        self.should_clear_screen = true;
        self.active_popup = None;
        self.input.0.reset();
        self.input.1.reset();
        match screen {
            Screen::Login => {
                self.curr_screen = Screen::Login;
                self.curr_screen_section = ScreenSection::Main;
                self.input_mode = InputMode::Editing(0);
                self.failed_logins = 0;
                self.client.active = None;
            }
            Screen::Client => {
                self.curr_screen = Screen::Client;
                self.curr_screen_section = ScreenSection::Main;
                self.input_mode = InputMode::Normal;
                self.help_text = "Choose an action to perform. Press Esc to go back."
            }
            Screen::Admin => {
                self.curr_screen = Screen::Admin;
                self.curr_screen_section = ScreenSection::Left;
                self.input_mode = InputMode::Normal;
                self.help_text = "Choose an action. `Alt`: Switch windows. `Esc`: Go back."
            }
            _ => { unimplemented!() }
        }
    }

    
    /// The timeout tick rate here should be equal or greater to the EventHandler tick rate.
    /// This is important because the minimum update time perceivable is defined by the EventHandler tick rate.
    pub fn add_timeout(&mut self, counter: u8, tick_rate: u16, timeout_type: TimeoutType) {
        if self.timeout.contains_key(&timeout_type) {
            panic!("cannot add timeout {:?} to list of timeouts. It already exists", timeout_type);
        }

        let tick_rate = Duration::from_millis(tick_rate as u64);

        self.timeout.insert(timeout_type, Timer{
            counter,
            tick_rate,
            last_update: Instant::now(),
        });
    }

    pub fn update_timeout_counter(&mut self, timeout_type: TimeoutType) {
        let timer = self.timeout.get_mut(&timeout_type)
            .unwrap_or_else(|| panic!("tried to update a nonexistent timeout"));

        if timer.counter > 1 {
            timer.counter -= 1;
            timer.last_update = Instant::now();
        } else {
            match timeout_type {
                TimeoutType::Login => self.failed_logins = 0,
                _ => {}
            }
            self.timeout.remove(&timeout_type);
        }
    }
}