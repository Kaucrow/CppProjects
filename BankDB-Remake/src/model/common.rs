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

pub trait ListItemTrait {
    fn len(&self) -> usize;
}

impl ListItemTrait for Vec<Popup> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
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

#[derive(Copy, Clone)]
pub enum Button {
    Up,
    Down
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CltData {
    Username,
    Name,
    Ci,
    AccNum,
    Balance,
    AccType,
    AccStatus,
    PsswdHash,
}

impl ListItemTrait for Vec<CltData> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl CltData {
    pub fn to_list_string(&self) -> &str {
        match self {
            CltData::Username => "Username",
            CltData::Name => "Name",
            CltData::Ci => "C.I.",
            CltData::AccNum => "Account number",
            CltData::Balance => "Balance",
            CltData::AccType => "Account type",
            CltData::AccStatus => "Account status",
            CltData::PsswdHash => "Password",
        }
    }

    pub fn as_sql_col(&self) -> String {
        match self {
            Self::Username => String::from("username"),
            Self::Name => String::from("name"),
            Self::Ci => String::from("ci"),
            Self::AccNum => String::from("account_number"),
            Self::Balance => String::from("balance"),
            Self::AccType => String::from("account_type"),
            Self::AccStatus => String::from("suspended"),
            Self::PsswdHash => String::from("password")
        }
    }
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
    CltData,
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
    AdminAddClient,
}

pub struct InputFields(pub Input, pub Input);

pub struct Timer {
    pub counter: u8,
    pub tick_rate: Duration,
    pub last_update: Instant,
}