use tui_input::Input;

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
    Editing,
}

/// Holds the state of input
pub struct InputData {
    /// Current value of the input box
    pub value: Input,
    /// Current input mode
    pub mode: InputMode,
    /// Last message recorded
    pub message: String,
}

pub struct App {
    pub input: InputData,
    pub curr_screen: Screen,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            input: InputData {
                value: Input::default(),
                mode: InputMode::Normal,
                message: String::new(),
            },
            curr_screen: Screen::Login,
            should_quit: false,
        }
    }
 
    pub fn change_screen(&mut self, screen: Screen) {
        match screen {
            Screen::Login => {
                self.curr_screen = Screen::Login;
                self.input.mode = InputMode::Editing;
                self.input.message.clear();
            }
            _ => { unimplemented!() }
        }
    }
}