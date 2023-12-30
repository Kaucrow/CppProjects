use tui_input::Input;
use std::time::{Instant, Duration};
use std::collections::HashMap;

pub enum AccountType {
    Debit,
    Current,
}

pub struct Client {
    ci: u32,
    name: String,
    password: String,
    account_number: u32,
    account_type: AccountType,
    suspended: bool,
}

pub enum Screen {
    Login,
}

pub enum InputMode {
    Normal,
    /// The value represents the InputField being edited
    Editing(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TimeoutType {
    Resize,
    Login,
}

pub struct InputFields(pub Input, pub Input);

pub struct Timer {
    pub counter: u8,
    pub tick_rate: Duration,
    pub last_update: Instant,
}

pub struct App {
    pub input: InputFields,
    pub input_mode: InputMode,
    pub failed_logins: u8,
    pub timeout: HashMap<TimeoutType, Timer>,
    pub resize_timeout: Instant,
    pub curr_screen: Screen,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            input: InputFields(Input::default(), Input::default()),
            input_mode: InputMode::Normal,
            failed_logins: 0,
            timeout: HashMap::new(),
            //timer_counter: None,
            //timer_step: None,
            resize_timeout: Instant::now(),
            curr_screen: Screen::Login,
            should_quit: false,
        }
    }
 
    pub fn change_screen(&mut self, screen: Screen) {
        match screen {
            Screen::Login => {
                self.curr_screen = Screen::Login;
                self.input_mode = InputMode::Editing(0);
                self.failed_logins = 0;
                self.input.0.reset();
                self.input.1.reset();
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