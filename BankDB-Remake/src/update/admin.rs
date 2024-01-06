use std::sync::{Arc, Mutex};
use sqlx::{Pool, Postgres};
use anyhow::Result;
use crate::{
    event::Event,
    model::app::{
        App,
        InputMode,
        Filter,
    },
};

pub async fn update(app: &mut Arc<Mutex<App>>, _: &Pool<Postgres>, event: Event) -> Result<()> {
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
        Event::RegisterFilter => {
            let mut app_lock = app.lock().unwrap();
            let filter = app_lock.admin.active_filter.unwrap();

            match filter {
                Filter::Username | Filter::Name |
                Filter::Ci | Filter::Balance | Filter::AccNum => {
                    let input_value = app_lock.input.0.value().to_string();
                    app_lock.admin.applied_filters.insert(filter, Some(input_value));
                }
                _ => {}
            }

            Ok(())
        }
        _ => panic!("An event of type {:?} was passed to the admin update function", event)
    }
}