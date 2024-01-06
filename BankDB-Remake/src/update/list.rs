use std::sync::{Arc, Mutex};
use sqlx::{Pool, Postgres};
use anyhow::Result;
use crate::{
    event::Event,
    model::app::{
        App,
        ScreenSection,
        Popup,
        InputMode,
        ListType,
    }
};

pub async fn update(app: &mut Arc<Mutex<App>>, _: &Pool<Postgres>, event: Event) -> Result<()> {
    match event {
        Event::NextListItem(list_type) => {
            app.lock().unwrap().next_list_item(list_type);
            Ok(())
        },
        Event::PreviousListItem(list_type) => {
            app.lock().unwrap().previous_list_item(list_type);
            Ok(())
        },
        Event::SelectAction(list_type) => {
            let mut app_lock = app.lock().unwrap();
            let (list_state, popups) = match list_type {
                ListType::ClientAction => (&app_lock.client.action_list_state, &app_lock.client.popups),
                ListType::AdminAction => (&app_lock.admin.action_list_state, &app_lock.admin.popups),
                _ => panic!()
            };

            if let Some(selected) = list_state.selected() {
                app_lock.active_popup = Some(*popups.get(&selected).unwrap_or_else(|| panic!("popup not found in popups HashMap")));
                match app_lock.active_popup.unwrap() {
                    Popup::Deposit | Popup::Withdraw | Popup::Transfer | Popup::ChangePsswd => app_lock.input_mode = InputMode::Editing(0),
                    Popup::FilterClients => app_lock.admin.filter_screen_section = ScreenSection::Left,
                    _ => {}
                }
            }
            Ok(())
        },
        _ => panic!("An event of type {:?} was passed to the wrong update function", event)
    }
}