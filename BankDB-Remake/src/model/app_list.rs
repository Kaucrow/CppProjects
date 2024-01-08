use anyhow::Result;
use sqlx::PgPool;
use crate::model::{
    common::{Filter, Button, ListType, TableType},
    app::App,
};

use super::admin::{ModifiedTable, GetClientsType};

impl App {
    pub async fn next_table_item(&mut self, table_type: TableType, pool: &PgPool) -> Result<()> {
        let mut modified_table = ModifiedTable::No;

        if let Some(selection) = self.admin.client_table_state.selected() {
            if selection >= self.admin.stored_clients.len() - 1 {
                modified_table = self.admin.get_clients(pool, GetClientsType::Next).await?;
            }
        }

        let (table_state, items) = match table_type {
            TableType::Clients => (&mut self.admin.client_table_state, &self.admin.stored_clients),
            _ => panic!()
        };

        let i = match table_state.selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    if let ModifiedTable::No = modified_table { i }
                    else { 0 }
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        table_state.select(Some(i));

        Ok(())
    }

    pub async fn previous_table_item(&mut self, table_type: TableType, pool: &PgPool) -> Result<()> {
        let mut modified_table = ModifiedTable::No;

        if let Some(selection) = self.admin.client_table_state.selected() {
            if selection == 0 {
                modified_table = self.admin.get_clients(pool, GetClientsType::Previous).await?;
            }
        }

        let (table_state, items) = match table_type {
            TableType::Clients => (&mut self.admin.client_table_state, &self.admin.stored_clients),
            _ => panic!()
        };

        let i = match table_state.selected() {
            Some(i) => {
                if i == 0 {
                    if let ModifiedTable::No = modified_table { 0 }
                    else { items.len() - 1}
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        table_state.select(Some(i));

        Ok(())
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
}