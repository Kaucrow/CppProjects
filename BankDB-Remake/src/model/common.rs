use std::time::{Duration, Instant};
use tui_input::Input;

#[derive(Debug, Clone)]
pub enum Screen {
    Login,
    Client,
    Admin,
}

#[derive(Copy, Clone)]
#[cfg_attr(feature = "debug_derive", derive(Debug))]
pub enum Popup {
    LoginSuccessful,
    ViewInfo,
    Deposit,
    Withdraw,
    Transfer,
    ChangePsswd,
    FilterClients,
    AddClient,
}

pub enum Button {
    Up,
    Down
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Filter {
    Username,
    Name,
    Ci,
    AccNum,
    Balance,
    AccType,
    AccStatus,
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

#[derive(Debug)]
pub enum ListType {
    ClientAction,
    AdminAction,
    ClientFilters,
}

#[derive(Debug)]
pub enum TableType {
    Clients 
}

#[derive(Debug)]
pub enum ScreenSection {
    Main,
    Left,
    Right,
}

#[derive(Debug)]
pub enum ScreenSectionType {
    AdminMain,
    AdminFilters,
}

pub struct InputFields(pub Input, pub Input);

pub struct Timer {
    pub counter: u8,
    pub tick_rate: Duration,
    pub last_update: Instant,
}