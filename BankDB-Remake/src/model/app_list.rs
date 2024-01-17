use anyhow::Result;
use sqlx::PgPool;
use super::{
    common::{ListItems, Popup, Button, ListType, TableType},
    admin::{CltField, GetClientsType, ModifiedTable},
    app::App,
};

impl App {
    pub async fn next_table_item(&mut self, table_type: TableType, pool: &PgPool) -> Result<()> {
        let mut modified_table = ModifiedTable::No;

        if let Some(selection) = self.admin.clients_table_state.selected() {
            if selection >= self.admin.stored_clients.len() - 1 {
                modified_table = self.admin.get_clients(pool, GetClientsType::Next).await?;
            }
        }

        let (table_state, items) = match table_type {
            TableType::Clients => (&mut self.admin.clients_table_state, &self.admin.stored_clients),
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

        if let Some(selection) = self.admin.clients_table_state.selected() {
            if selection == 0 {
                modified_table = self.admin.get_clients(pool, GetClientsType::Previous).await?;
            }
        }

        let (table_state, items) = match table_type {
            TableType::Clients => (&mut self.admin.clients_table_state, &self.admin.stored_clients),
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
        let (list_state, items): (_, &dyn ListItems) = match list_type {
            ListType::ClientAction => (&mut self.client.actions_list_state, &self.client.actions),
            ListType::AdminAction => (&mut self.admin.action_list_state, &self.admin.actions),
            ListType::CltField => (&mut self.admin.cltfields_list_state, &self.admin.cltfields),
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

        if let ListType::CltField = list_type {
            self.update_cltfield_data(i);
        }
    }
    
    pub fn previous_list_item(&mut self, list_type: ListType) {
        let (list_state, items): (_, &dyn ListItems) = match list_type {
            ListType::ClientAction => (&mut self.client.actions_list_state, &self.client.actions),
            ListType::AdminAction => (&mut self.admin.action_list_state, &self.admin.actions),
            ListType::CltField => (&mut self.admin.cltfields_list_state, &self.admin.cltfields),
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
        
        if let ListType::CltField = list_type {
            self.update_cltfield_data(i);
        }
    }

    fn update_cltfield_data(&mut self, list_selection: usize) {
        let cltfield = *self.admin.cltfields.get(list_selection)
            .unwrap_or_else(|| panic!("sidescreen not found in filter sidescreens"));

        self.admin.active_cltfield = Some(cltfield);

        let registered_cltfield = match self.active_popup {
            Some(Popup::FilterClients) => &self.admin.applied_filters,
            Some(Popup::AddClient) => &self.admin.registered_cltfields,
            _ => panic!("fn update_cltfield_data was called on a popup of type {:?}", self.active_popup)
        };
        
        if let Some(value) = registered_cltfield.get(&cltfield).unwrap() {
            match cltfield {
                CltField::Username | CltField::Name | CltField::Ci |
                CltField::Balance | CltField::AccNum
                => self.input.0 = value.clone().into(),

                CltField::AccStatus => {
                    if value == "suspended" {
                        self.admin.button_selection = Some(Button::Up)
                    } else {
                        self.admin.button_selection = Some(Button::Down)
                    }
                },

                CltField::AccType => {
                    if value == "current" {
                        self.admin.button_selection = Some(Button::Up)
                    } else {
                        self.admin.button_selection = Some(Button::Down)
                    }
                }

                _ => {}
            }
        } else {
            match cltfield {
                CltField::Username | CltField::Name | CltField::Ci |
                CltField::Balance | CltField::AccNum
                => self.input.0.reset(),

                CltField::AccStatus | CltField::AccType
                => self.admin.button_selection = None,
                
                _ => {}
            }
        }
    }
}