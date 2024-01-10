use std::sync::{Arc, Mutex};
use sqlx::{Pool, Postgres};
use rust_decimal::Decimal;
use bcrypt::{verify, hash};
use anyhow::Result;
use crate::{
    HELP_TEXT,
    event::Event,
    model::app::App,
    update::common_fn::modify_balance
};

pub async fn update(app: &mut Arc<Mutex<App>>, pool: &Pool<Postgres>, event: Event) -> Result<()> {
    match event {
        Event::Deposit | Event::Withdraw => {
            modify_balance(app, pool, event).await?;
            Ok(())
        },
        Event::Transfer => {
            let beneficiary = app.lock().unwrap().input.1.value().to_string();
            {
                let mut app_lock = app.lock().unwrap();
                if beneficiary == app_lock.client.active.as_ref().unwrap().username {
                    app_lock.help_text = HELP_TEXT.client.transfer_to_self;
                    app_lock.hold_popup = true;
                    return Ok(());
                }
            }

            match sqlx::query("SELECT * FROM clients WHERE username = $1")
                .bind(&beneficiary)
                .fetch_optional(pool)
                .await? {
                    Some(_) => {
                        let prev_balance = app.lock().unwrap().client.active.as_ref().unwrap().balance.clone();

                        modify_balance(app, pool, event).await?;

                        let mut app_lock = app.lock().unwrap();
                        if app_lock.client.active.as_ref().unwrap().balance != prev_balance {
                            sqlx::query("UPDATE clients SET balance = balance + $1 WHERE username = $2")
                            .bind(Decimal::from_str_exact(app_lock.input.0.value()).unwrap())
                            .bind(beneficiary)
                            .execute(pool)
                            .await?;

                            app_lock.input.0.reset();
                            app_lock.input.1.reset();
                            app_lock.active_popup = None;
                        }
                    },
                    None => {
                        let mut app_lock = app.lock().unwrap();
                        app_lock.help_text = HELP_TEXT.client.unknown_beneficiary;
                        app_lock.hold_popup = true;
                    }
                }
            Ok(())
        },
        Event::ChangePasswd => {
            let mut app_lock = app.lock().unwrap();
            let curr_passwd = &app_lock.client.active.as_ref().unwrap().password_hash;
            let curr_passwd_input = app_lock.input.0.value();

            if verify(curr_passwd_input, curr_passwd).unwrap_or_else(|error| panic!("{}", error)) {
                let new_password_hash = hash(
                    app_lock.input.1.value(), 4).unwrap_or_else(|error| panic!("{}", error
                ));

                sqlx::query("UPDATE clients SET password = $1 WHERE username = $2")
                    .bind(&new_password_hash)
                    .bind(&app_lock.client.active.as_ref().unwrap().username)
                    .execute(pool)
                    .await?;

                app_lock.client.active.as_mut().unwrap().password_hash = new_password_hash;

                app_lock.input.0.reset();
                app_lock.input.1.reset();
                app_lock.active_popup = None;
            } else {
                app_lock.help_text = HELP_TEXT.client.incorrect_password;
                app_lock.hold_popup = true;
            }

            Ok(())      
        },
        _ => panic!("An event of type {:?} was passed to the client update function", event)
    }
}