use tui_input::Input;
use std::time::{Instant, Duration};
use std::collections::HashMap;
use ratatui::widgets::{ListState, TableState, Table};
use crate::model::client::Client;

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

pub struct ClientData {
    pub active: Option<Client>,
    pub actions: Vec<&'static str>,
    pub action_list_state: ListState,
    pub popups: HashMap<usize, Popup>,
}

impl std::default::Default for ClientData {
    fn default() -> Self {
        ClientData {
            active: None,
            actions: vec![
                "View info",
                "Make a deposit",
                "Make a withdrawal",
                "Make a transfer",
                "Change password"
            ],
            action_list_state: ListState::default(),
            popups: HashMap::from([
                (0, Popup::ViewInfo),
                (1, Popup::Deposit),
                (2, Popup::Withdraw),
                (3, Popup::Transfer),
                (4, Popup::ChangePsswd)
            ])
        }
    }
}

pub enum Button {
    Up,
    Down
}

pub struct AdminData {
    pub actions: Vec<&'static str>,
    pub action_list_state: ListState,
    pub client_table_state: TableState,
    pub stored_clients: Vec<Client>,
    pub popups: HashMap<usize, Popup>,
    pub filters: Vec<&'static str>,
    pub filter_sidescreens: HashMap<usize, Filter>,
    pub filter_list_state: ListState,
    pub filter_screen_section: ScreenSection,
    pub active_filter: Option<Filter>,
    pub applied_filters: HashMap<Filter, Option<String>>,
    pub button_selection: Option<Button>,
}

impl std::default::Default for AdminData {
    fn default() -> Self {
        AdminData {
            actions: vec![
                "Filter clients",
                "Add a client"
            ],
            action_list_state: ListState::default(),
            client_table_state: TableState::default(),
            stored_clients: Vec::new(),
            popups: HashMap::from([
                (0, Popup::FilterClients),
                (1, Popup::AddClient)
            ]),
            filters: vec![
                "Username",
                "Name",
                "C.I.",
                "Account number",
                "Balance",
                "Account type",
                "Account status",
            ],
            filter_sidescreens: HashMap::from([
                (0, Filter::Username),
                (1, Filter::Name),
                (2, Filter::Ci),
                (3, Filter::AccNum),
                (4, Filter::Balance),
                (5, Filter::AccType),
                (6, Filter::AccStatus),
            ]),
            filter_list_state: ListState::default(),
            filter_screen_section: ScreenSection::Left,
            active_filter: None,
            applied_filters: HashMap::from([
                (Filter::Username, None),
                (Filter::Name, None),
                (Filter::Ci, None),
                (Filter::AccNum, None),
                (Filter::Balance, None),
                (Filter::AccType, None),
                (Filter::AccStatus, None),
            ]),
            button_selection: None,
        }
    }
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
                self.help_text = "Choose a client or an action. Press Alt to switch windows. Press Esc to go back."
            }
            _ => { unimplemented!() }
        }
    }

    pub fn next_table_item(&mut self, table_type: TableType) {
        let (table_state, items) = match table_type {
            TableType::Clients => (&mut self.admin.client_table_state, &self.admin.stored_clients),
            _ => panic!()
        };

        let i = match table_state.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        table_state.select(Some(i));
    }

    pub fn previous_table_item(&mut self, table_type: TableType) {
        let (table_state, items) = match table_type {
            TableType::Clients => (&mut self.admin.client_table_state, &self.admin.stored_clients),
            _ => panic!()
        };

        let i = match table_state.selected() {
            Some(i) => {
                if i == 0 {
                    i
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        table_state.select(Some(i));
    }

    pub fn next_list_item(&mut self, list_type: ListType) {
        let (list_state, items) = match list_type {
            ListType::ClientAction => (&mut self.client.action_list_state, &self.client.actions),
            ListType::AdminAction => (&mut self.admin.action_list_state, &self.admin.actions),
            ListType::ClientFilters => (&mut self.admin.filter_list_state, &self.admin.filters),
            _ => panic!()
        };

        let i = match list_state.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        list_state.select(Some(i));

        if let ListType::ClientFilters = list_type {
            self.update_filter_data(i);
        }
    }
    
    pub fn previous_list_item(&mut self, list_type: ListType) {
        let (list_state, items) = match list_type {
            ListType::ClientAction => (&mut self.client.action_list_state, &self.client.actions),
            ListType::AdminAction => (&mut self.admin.action_list_state, &self.admin.actions),
            ListType::ClientFilters => (&mut self.admin.filter_list_state, &self.admin.filters),
            _ => panic!()
        };

        let i = match list_state.selected() {
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        list_state.select(Some(i));
        
        if let ListType::ClientFilters = list_type {
            self.update_filter_data(i);
        }
    }

    fn update_filter_data(&mut self, list_selection: usize) {
        let filter = *self.admin.filter_sidescreens.get(&list_selection)
            .unwrap_or_else(|| panic!("sidescreen not found in filter sidescreens"));

        self.admin.active_filter = Some(filter);
        
        if let Some(value) = self.admin.applied_filters.get(&filter).unwrap() {
            match filter {
                Filter::Username | Filter::Name | Filter::Ci |
                Filter::Balance | Filter::AccNum
                => self.input.0 = value.clone().into(),

                Filter::AccStatus => {
                    if value == "suspended" {
                        self.admin.button_selection = Some(Button::Up)
                    } else {
                        self.admin.button_selection = Some(Button::Down)
                    }
                },

                Filter::AccType => {
                    if value == "current" {
                        self.admin.button_selection = Some(Button::Up)
                    } else {
                        self.admin.button_selection = Some(Button::Down)
                    }
                }
            }
        } else {
            match filter {
                Filter::Username | Filter::Name | Filter::Ci |
                Filter::Balance | Filter::AccNum
                => self.input.0.reset(),

                Filter::AccStatus | Filter::AccType
                => self.admin.button_selection = None
            }
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