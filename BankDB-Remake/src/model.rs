use tui_input::Input;
use std::time::Instant;
use sqlx::{ Pool, Postgres };

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

pub struct InputFields(pub Input, pub Input);

pub struct App {
    //pub pool: Pool<Postgres>,
    pub input: InputFields,
    pub input_mode: InputMode,
    pub resize_timeout: Instant,
    pub curr_screen: Screen,
    pub should_quit: bool,
}

impl App {
    pub fn new(/*pool: Pool<Postgres>*/) -> Self {
        App {
            //pool,
            input: InputFields(Input::default(), Input::default()),
            input_mode: InputMode::Normal,
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
                self.input.0.reset();
                self.input.1.reset();
            }
            _ => { unimplemented!() }
        }
    }
}