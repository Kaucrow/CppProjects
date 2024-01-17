use std::{fs, collections::HashMap};
use anyhow::Result;
use sqlx::{FromRow, PgPool};
use ratatui::widgets::{ListState, TableState};
use crate::{
    DATA_PATH,
    model::{
        common::{Popup, SideScreen, Button, ScreenSection, ListItems},
        client::Client,
    }
};

pub struct AdminData {
    pub actions: Vec<Popup>,
    pub action_list_state: ListState,
    pub clients_table_state: TableState,
    pub stored_clients: Vec<Client>,
    pub viewing_clients: i32,
    pub query_clients: String,
    pub cltfields: Vec<CltField>,
    pub cltfields_list_state: ListState,
    pub popup_screen_section: ScreenSection,
    pub button_selection: Option<Button>,
    pub active_cltfield: Option<CltField>,
    pub applied_filters: HashMap<CltField, Option<String>>,
    pub registered_cltfields: HashMap<CltField, Option<String>>,
    pub active_sidescreen: SideScreen,
    pub user_logo: String,
    pub client_edit_fields: Vec<&'static str>
}

impl std::default::Default for AdminData {
    fn default() -> Self {
        AdminData {
            actions: vec![
                Popup::FilterClients,
                Popup::AddClient,
            ],
            action_list_state: ListState::default(),
            clients_table_state: TableState::default(),
            stored_clients: Vec::new(),
            viewing_clients: 0,
            query_clients: String::from("SELECT * FROM clients"),
            cltfields: vec![
                CltField::Username,
                CltField::Name,
                CltField::Ci,
                CltField::AccNum,
                CltField::AccType,
                CltField::AccStatus,
            ],
            cltfields_list_state: ListState::default(),
            popup_screen_section: ScreenSection::Left,
            button_selection: None,
            active_cltfield: None,
            applied_filters: HashMap::from([
                (CltField::Username, None),
                (CltField::Name, None),
                (CltField::Ci, None),
                (CltField::AccNum, None),
                (CltField::Balance, None),
                (CltField::AccType, None),
                (CltField::AccStatus, None),
            ]),
            registered_cltfields: HashMap::from([
                (CltField::Username, None),
                (CltField::Name, None),
                (CltField::Ci, None),
                (CltField::AccNum, None),
                (CltField::Balance, None),
                (CltField::AccType, None),
                (CltField::AccStatus, None),
                (CltField::PsswdHash, None),
            ]),
            active_sidescreen: SideScreen::AdminClientTable,
            user_logo: fs::read_to_string(format!("{}user_logo.txt", DATA_PATH.lock().unwrap())).unwrap(),
            client_edit_fields: vec![
                "Username: ",
                "C.I.: ",
                "Account num.: ",
                "Balance: ",
                "Account type: ",
                "Last transaction: ",
                "Account status: ",
            ]
        }
    }
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CltField {
    Username,
    Name,
    Ci,
    AccNum,
    Balance,
    AccType,
    AccStatus,
    PsswdHash,
}

impl ListItems for Vec<CltField> {
    fn len(&self) -> usize {
        Vec::len(self)
    }
}

impl CltField {
    pub fn to_list_string(&self) -> &str {
        match self {
            CltField::Username => "Username",
            CltField::Name => "Name",
            CltField::Ci => "C.I.",
            CltField::AccNum => "Account number",
            CltField::Balance => "Balance",
            CltField::AccType => "Account type",
            CltField::AccStatus => "Account status",
            CltField::PsswdHash => "Password",
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

pub enum ModifiedTable {
    Yes,
    No
}

pub enum GetClientsType {
    Next,
    Previous
}

#[derive(Debug, PartialEq)]
pub enum CltFieldType {
    CltField,
    Filter
}