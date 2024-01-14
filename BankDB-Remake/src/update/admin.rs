use std::sync::{Arc, Mutex};
use rust_decimal::Decimal;
use sqlx::{Pool, Postgres, query::Query, postgres::PgArguments};
use anyhow::Result;
use bcrypt::hash;
use crate::{
    event::Event,
    model::{
        common::{Popup, InputMode, CltData, Button},
        app::App, admin::CltDataType,
    }, HELP_TEXT,
};

pub async fn update(app: &mut Arc<Mutex<App>>, pool: &Pool<Postgres>, event: Event) -> Result<()> {
    match event {
        Event::EditCltData => {
            let mut app_lock = app.lock().unwrap();
            match app_lock.admin.active_cltdata {
                Some(CltData::Username) | Some(CltData::Name) |
                Some(CltData::Ci) | Some(CltData::Balance) | Some(CltData::AccNum) =>
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
        Event::RegisterCltData(cltdata_type) => {
            let mut app_lock = app.lock().unwrap();
            let cltdata = app_lock.admin.active_cltdata.unwrap();

            let input0 = app_lock.input.0.value().to_string();
            let button_selection = app_lock.admin.button_selection.clone();

            let registered_cltdata = match cltdata_type {
                CltDataType::Filter => &mut app_lock.admin.applied_filters,
                CltDataType::CltData => &mut app_lock.admin.registered_cltdata,
            };

            match cltdata {
                CltData::Username | CltData::Name | CltData::Ci |
                CltData::Balance | CltData::AccNum
                => {
                    registered_cltdata.insert(cltdata, Some(input0));
                }

                CltData::AccStatus => {
                    match button_selection {
                        Some(Button::Up) => { registered_cltdata.insert(cltdata, Some("suspended".to_string())); },
                        Some(Button::Down) => { registered_cltdata.insert(cltdata, Some("not suspended".to_string())); },
                        _ => {}
                    }
                }
                
                CltData::AccType => {
                    match button_selection {
                        Some(Button::Up) => { registered_cltdata.insert(cltdata, Some("current".to_string())); },
                        Some(Button::Down) => { registered_cltdata.insert(cltdata, Some("debit".to_string())); },
                        _ => {}
                    }
                }

                CltData::PsswdHash => {
                    registered_cltdata.insert(cltdata, Some(hash(input0, 4)?));
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
                        CltData::Username => query.push_str(format!("username = '{value}' AND ").as_str()),
                        CltData::Name => query.push_str(format!("name = '{value}' AND ").as_str()),
                        CltData::Ci => query.push_str(format!("ci = '{value}' AND ").as_str()),
                        CltData::AccNum => query.push_str(format!("account_number = '{value}' AND ").as_str()),
                        CltData::Balance => query.push_str(format!("balance = '{value}' AND ").as_str()),
                        CltData::AccType => query.push_str(format!("account_type = '{value}' AND ").as_str()),
                        CltData::AccStatus => match value.as_str() {
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
            app_lock.admin.get_clients_raw(pool, true).await?;

            Ok(())
        }
        Event::CheckAddClient => {
            let mut app_lock = app.lock().unwrap();

            for (cltdata, value) in app_lock.admin.registered_cltdata.iter() {
                if *cltdata != CltData::PsswdHash {
                    if value.is_none() {
                        app_lock.help_text = format!("{}{:?}", HELP_TEXT.admin.missing_cltdata, cltdata);
                        app_lock.hold_popup = true;
                        return Ok(());
                    }
                    else if matches!(cltdata, CltData::Username | CltData::Ci | CltData::AccNum) {
                        let query_base = format!("SELECT * FROM clients WHERE {} = $1", cltdata.as_sql_col());

                        let mut query: Query<'_, Postgres, PgArguments> = sqlx::query(&query_base.as_str());

                        if let Ok(parsed_value) = value.as_ref().unwrap().parse::<i64>() {
                            query = query.bind(parsed_value);
                        } else {
                            query = query.bind(value);
                        }

                        if query
                            .fetch_optional(pool)
                            .await?
                            .is_some() {
                                app_lock.help_text = format!("{:?} already exists.", cltdata);
                                app_lock.hold_popup = true;
                                return Ok(());
                            }
                    }
                }                         
            }

            app_lock.switch_popup = Some(Popup::AddClientPsswd);

            Ok(())
        }
        Event::AddClient => {
            let mut app_lock = app.lock().unwrap();

            let psswd_hash = hash(app_lock.input.0.value(), 4)?;

            app_lock.admin.registered_cltdata.insert(CltData::PsswdHash, Some(psswd_hash));

            let mut query_text = String::from("INSERT INTO clients (");

            app_lock.admin.registered_cltdata.keys()
                .for_each(|cltdata| query_text.push_str(format!("{},", cltdata.as_sql_col()).as_str()));

            query_text.pop();
            query_text.push_str(") VALUES ($1,$2,$3,$4,$5,$6,$7,$8)");

            let mut query: Query<'_, Postgres, PgArguments> = sqlx::query(query_text.as_str());

            for (cltdata, value) in app_lock.admin.registered_cltdata.iter() {
                if let Some(value) = value {
                    match cltdata {
                        CltData::Ci | CltData::AccNum =>
                            query = query.bind(value.parse::<i64>().unwrap()),
                        
                        CltData::Balance =>
                            query = query.bind(Decimal::from_str_exact(value).unwrap()),

                        CltData::AccStatus =>
                            match value.as_str() {
                                "suspended" => query = query.bind(true),
                                "not suspended" => query = query.bind(false),
                                _ => panic!("unknown value found on {:?}", cltdata)
                            }

                        _ =>
                            query = query.bind(value)
                    }
                } else {
                    query = query.bind("null,");
                }
            }

            query.execute(pool).await?;

            app_lock.switch_popup = Some(Popup::AddClientSuccess);

            Ok(())
        }
        _ => panic!("An event of type {:?} was passed to the admin update function", event)
    }
}