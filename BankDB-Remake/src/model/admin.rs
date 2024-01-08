use std::collections::HashMap;
use ratatui::widgets::{ListState, TableState};
use crate::model::{
    common::{Popup, Filter, Button, ScreenSection},
    client::Client,
};

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