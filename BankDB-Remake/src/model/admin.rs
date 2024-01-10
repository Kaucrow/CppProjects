use std::collections::HashMap;
use anyhow::Result;
use sqlx::{FromRow, PgPool};
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
    pub viewing_clients: i32,
    pub query_clients: String,
    pub popups: HashMap<usize, Popup>,
    pub filters: Vec<&'static str>,
    pub filter_sidescreens: HashMap<usize, Filter>,
    pub filter_list_state: ListState,
    pub popup_screen_section: ScreenSection,
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
            viewing_clients: 0,
            query_clients: String::from("SELECT * FROM clients"),
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
            popup_screen_section: ScreenSection::Left,
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

pub enum ModifiedTable {
    Yes,
    No
}

pub enum GetClientsType {
    Next,
    Previous
}

impl AdminData {
    pub async fn get_clients(&mut self, pool: &PgPool, get_type: GetClientsType) -> Result<ModifiedTable> {
        match get_type {
            GetClientsType::Next => self.viewing_clients += 10,
            GetClientsType::Previous => {
                if self.viewing_clients == 0 { return Ok(ModifiedTable::No); }
                self.viewing_clients -= 10;
            }
        }

        self.query_clients.push_str(format!(" LIMIT 10 OFFSET {}", self.viewing_clients).as_str());
        let result = self.get_clients_raw(pool, false).await?;
        self.query_clients.truncate(self.query_clients.find(" LIMIT").unwrap());

        if let (GetClientsType::Next, ModifiedTable::No) = (get_type, &result) {
            self.viewing_clients += 10
        }

        Ok(result)
    }

    pub async fn get_clients_raw(&mut self, pool: &PgPool, store_if_res_empty: bool) -> Result<ModifiedTable> {
        let res: Vec<Client> = {
            sqlx::query(self.query_clients.as_str())
            .fetch_all(pool)
            .await?
            .iter()
            .map(|row| { Client::from_row(row) } )
            .collect::<Result<_, sqlx::Error>>()?
        };

        if !res.is_empty() || store_if_res_empty {
            self.stored_clients.clear();
            self.stored_clients = res;
            return Ok(ModifiedTable::Yes);
        }

        Ok(ModifiedTable::No)
    }
}