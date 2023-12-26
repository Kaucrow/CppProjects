use std::collections::HashMap;
use std::collections::BTreeMap;

use ratatui::widgets::List;

use anyhow::Result;
use ratatui::widgets::ListState;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    // The currently being edited json key
    pub key_input: String,

    // The currently being edited json value
    pub value_input: String,

    // The representation of our key and value pairs with serde serialize support
    pub pairs: BTreeMap<String, String>,

    // The current screen the user is looking at, and will later
    // determine what is rendered
    pub current_screen: CurrentScreen,

    // The optional state containing which of the key or value
    // pair the user is editing. It is an option, because when
    // the user is not directly editing a key-value pair, this will be set to `None`
    pub currently_editing: Option<CurrentlyEditing>,

    pub list_state: ListState,
}

impl App {
    pub fn new() -> App {
        let mut pairs: BTreeMap<String, String> = BTreeMap::new();
        for (lower, upper) in ('a'..='z').zip('A'..='Z') {
            pairs.insert(upper.to_string(), upper.to_string());
        }

        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            list_state: ListState::default(),
        }
    }
    
    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.pairs.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.pairs.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => {
                    self.currently_editing = Some(CurrentlyEditing::Value)
                }
                CurrentlyEditing::Value => {
                    self.currently_editing = Some(CurrentlyEditing::Key)
                }
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn print_json(&self) -> Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}