use std::sync::{Arc, Mutex};
use sqlx::{Pool, Postgres, FromRow};
use anyhow::Result;
use crate::{
    event::Event,
    model::{
        common::{InputMode, Filter, Button},
        app::App,
        admin::GetClientsType
    },
};

pub async fn update(app: &mut Arc<Mutex<App>>, pool: &Pool<Postgres>, event: Event) -> Result<()> {
    match event {
        Event::EditFilter => {
            let mut app_lock = app.lock().unwrap();
            match app_lock.admin.active_filter {
                Some(Filter::Username) | Some(Filter::Name) |
                Some(Filter::Ci) | Some(Filter::Balance) | Some(Filter::AccNum) =>
                app_lock.input_mode = InputMode::Editing(0),
                _ => {}
            }
            Ok(())
        },
        Event::SwitchButton => {
            let mut app_lock = app.lock().unwrap();
            let button_selection = &mut app_lock.admin.button_selection;

            *button_selection = if let Some(Button::Up) = button_selection {
                Some(Button::Down)
            } else {
                Some(Button::Up)
            };

            Ok(())
        },
        Event::RegisterFilter => {
            let mut app_lock = app.lock().unwrap();
            let filter = app_lock.admin.active_filter.unwrap();

            match filter {
                Filter::Username | Filter::Name | Filter::Ci |
                Filter::Balance | Filter::AccNum
                => {
                    let input_value = app_lock.input.0.value().to_string();
                    app_lock.admin.applied_filters.insert(filter, Some(input_value));
                }

                Filter::AccStatus => {
                    match app_lock.admin.button_selection {
                        Some(Button::Up) => { app_lock.admin.applied_filters.insert(filter, Some("suspended".to_string())); },
                        Some(Button::Down) => { app_lock.admin.applied_filters.insert(filter, Some("not suspended".to_string())); },
                        _ => {}
                    }
                }
                
                Filter::AccType => {
                    match app_lock.admin.button_selection {
                        Some(Button::Up) => { app_lock.admin.applied_filters.insert(filter, Some("current".to_string())); },
                        Some(Button::Down) => { app_lock.admin.applied_filters.insert(filter, Some("debit".to_string())); },
                        _ => {}
                    }
                }
                _ => {}
            }

            Ok(())
        }
        Event::ApplyFilters => {
            let mut app_lock = app.lock().unwrap();

            let mut query = String::from("SELECT * FROM clients WHERE ");
            for (filter, value) in app_lock.admin.applied_filters.iter() {
                if value.is_some() {
                    let value = value.as_ref().unwrap();
                    match filter {
                        Filter::Username => query.push_str(format!("username = '{value}' AND ").as_str()),
                        Filter::Name => query.push_str(format!("name = '{value}' AND ").as_str()),
                        Filter::Ci => query.push_str(format!("ci = '{value}' AND ").as_str()),
                        Filter::AccNum => query.push_str(format!("account_number = '{value}' AND ").as_str()),
                        Filter::Balance => query.push_str(format!("balance = '{value}' AND ").as_str()),
                        Filter::AccType => query.push_str(format!("account_type = '{value}' AND ").as_str()),
                        Filter::AccStatus => match value.as_str() {
                            "suspended" => query.push_str("suspended = true AND "),
                            "not suspended" => query.push_str("suspended = false AND "),
                            _ => panic!("invalid applied filter value")
                        }
                        _ => {}
                    }
                }
            }

            query.pop();
            if let Some(last_space_idx) = query.rfind(' ') {
                query.truncate(last_space_idx);
            }

            app_lock.admin.query_clients = query;
            app_lock.admin.viewing_clients = 0;
            app_lock.admin.get_clients_raw(pool).await?;

            Ok(())
        }
        _ => panic!("An event of type {:?} was passed to the admin update function", event)
    }
}