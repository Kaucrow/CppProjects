use tui_input::Input;
use std::time::{Instant, Duration};
use std::collections::HashMap;
use rust_decimal::Decimal;
use sqlx::{postgres::PgRow, Row, FromRow};

pub enum AccountType {
    Debit,
    Current,
}

pub struct Transfer {
    amount: Decimal,
    recipient: String,
}

pub enum Transaction {
    Deposit(Decimal),
    Withdraw(Decimal),
    Transfer(Transfer),
}

impl<'r> FromRow<'r, PgRow> for Transaction {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let amount: Decimal = row.try_get("amount")?;
        let recipient: Option<String> = row.try_get("recipient")?;
        match row.try_get("operation")? {
            "deposit" => Ok(Transaction::Deposit(amount)),
            "withdraw" => Ok(Transaction::Withdraw(amount)),
            "transfer" => Ok(Transaction::Transfer(Transfer {
                amount,
                recipient: recipient.unwrap_or("recipient not found".to_string()),
            })),
            _ => Err(sqlx::Error::Decode(Box::new(sqlx::error::Error::ColumnDecode {
                index: "operation".to_string(),
                source: Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "invalid operation type"
                ))
            })))
        }
    }
}

pub struct Client {
    pub account_number: i32,
    pub username: String,
    pub name: String,
    pub ci: i32,
    pub account_type: AccountType,
    pub balance: Decimal,
    pub last_transaction: Option<Transaction>,
    pub suspended: bool,
}

impl Client {
    fn new() -> Self {
        Client {
            account_number: 0,
            username: String::new(),
            name: String::new(),
            ci: 0,
            account_type: AccountType::Current,
            balance: Decimal::new(0, 2),//Money{integer: 0, decimal: 0},
            last_transaction: None,
            suspended: false,
        }
    }
}

pub enum Screen {
    Login,
}

pub enum InputMode {
    Normal,
    /// The value represents the InputField being edited
    Editing(u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum TimeoutType {
    Resize,
    Login,
}

pub struct InputFields(pub Input, pub Input);

pub struct Timer {
    pub counter: u8,
    pub tick_rate: Duration,
    pub last_update: Instant,
}

pub struct App {
    pub input: InputFields,
    pub input_mode: InputMode,
    pub failed_logins: u8,
    pub active_user: Option<Client>,
    pub timeout: HashMap<TimeoutType, Timer>,
    pub curr_screen: Screen,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            input: InputFields(Input::default(), Input::default()),
            input_mode: InputMode::Normal,
            failed_logins: 0,
            active_user: None,
            timeout: HashMap::new(),
            curr_screen: Screen::Login,
            should_quit: false,
        }
    }
 
    pub fn change_screen(&mut self, screen: Screen) {
        match screen {
            Screen::Login => {
                self.curr_screen = Screen::Login;
                self.input_mode = InputMode::Editing(0);
                self.failed_logins = 0;
                self.active_user = None;
                self.input.0.reset();
                self.input.1.reset();
            }
            _ => { unimplemented!() }
        }
    }

    /// The timeout tick rate here should be equal or greater to the EventHandler tick rate.
    /// This is important because the minimum update time perceivable is defined by the EventHandler tick rate.
    pub fn add_timeout(&mut self, counter: u8, tick_rate: u16, timeout_type: TimeoutType) {
        if self.timeout.contains_key(&timeout_type) {
            panic!("cannot add timeout {:?} to list of timeouts. It already exists", timeout_type);
        }

        let tick_rate = Duration::from_millis(tick_rate as u64);

        self.timeout.insert(timeout_type, Timer{
            counter,
            tick_rate,
            last_update: Instant::now(),
        });
    }

    pub fn update_timeout_counter(&mut self, timeout_type: TimeoutType) {
        let timer = self.timeout.get_mut(&timeout_type)
            .unwrap_or_else(|| panic!("tried to update a nonexistent timeout"));

        if timer.counter > 1 {
            timer.counter -= 1;
            timer.last_update = Instant::now();
        } else {
            match timeout_type {
                TimeoutType::Login => self.failed_logins = 0,
                _ => {}
            }
            self.timeout.remove(&timeout_type);
        }
    }
}