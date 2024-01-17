use std::time::{Duration, Instant};
use tui_input::Input;

#[derive(Debug, Clone)]
pub enum Screen {
    Login,
    Client,
    Admin,
}

pub enum SideScreen {
    AdminClientTable,
    AdminClientEdit,
}

#[derive(Debug)]
pub enum ScreenSection {
    Main,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Copy, Clone)]
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
    AddClientPsswd,
    AddClientSuccess,
}

impl Popup {
    pub fn to_list_string(&self) -> &str {
        match self {
            Popup::ViewInfo => "View info",
            Popup::Deposit => "Make a deposit",
            Popup::Withdraw => "Make a withdrawal",
            Popup::Transfer => "Make a transfer",
            Popup::ChangePsswd => "Change password",
            Popup::FilterClients => "Filter clients",
            Popup::AddClient => "Add a client",
            _ => panic!("could not find a list string for popup {:?}", self)
        } 
    }
}

pub trait ListItems {
    fn len(&self) -> usize;
}

impl ListItems for Vec<Popup> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

#[derive(Copy, Clone)]
pub enum Button {
    Up,
    Down
}



pub enum InputMode {
    Normal,
    /// The value represents the InputField being edited
    Editing(u8),
}
pub struct InputFields(pub Input, pub Input);

pub struct Timer {
    pub counter: u8,
    pub tick_rate: Duration,
    pub last_update: Instant,
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
    CltField,
}

#[derive(Debug)]
pub enum TableType {
    Clients 
}

#[derive(Debug)]
pub enum ScreenSectionType {
    AdminMain,
    AdminFilters,
    AdminAddClient,
}