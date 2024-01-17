use std::sync::{Arc, Mutex};
use sqlx::{Pool, Postgres};
use rust_decimal::Decimal;
use anyhow::Result;
use crate::{
    HELP_TEXT,
    event::Event,
    model::app::App,
};

pub async fn modify_balance(app: &mut Arc<Mutex<App>>, pool: &Pool<Postgres>, event: Event) -> Result<()> {
    let mut app_lock = app.lock().unwrap();
    let amount_input = Decimal::from_str_exact(app_lock.input.0.value())?;

    if let Event::Deposit = event {
        app_lock.client.active.as_mut().unwrap().balance += amount_input;
    } else {
        if amount_input > app_lock.client.active.as_ref().unwrap().balance {
            app_lock.help_text = HELP_TEXT.client.not_enough_money.to_string();
            app_lock.hold_popup = true;
            return Ok(())
        } else {
            app_lock.client.active.as_mut().unwrap().balance -= amount_input;
        }
    }

    sqlx::query("UPDATE clients SET balance = $1 WHERE username = $2")
        .bind(&app_lock.client.active.as_ref().unwrap().balance)
        .bind(&app_lock.client.active.as_ref().unwrap().username)
        .execute(pool)
        .await?;

    app_lock.active_popup = None;
    Ok(())
}