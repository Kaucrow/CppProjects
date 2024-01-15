use std::collections::HashMap;
use ratatui::widgets::ListState;
use anyhow::Result;
use sqlx::postgres::PgTypeInfo;
use rust_decimal::Decimal;
use sqlx::{
    database::HasValueRef,
    postgres::PgRow,
    Postgres,
    PgPool,
    Row,
    FromRow,
    Decode,
    Type,
};
use crate::model::common::{Popup, CltData};

#[derive(Debug, Clone)]
pub enum AccountType {
    Debit,
    Current,
}

impl<'r> Decode<'r, Postgres> for AccountType
where
    &'r str: Decode<'r, Postgres>,
{
    fn decode(
        value: <Postgres as HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let value = <&str as Decode<Postgres>>::decode(value);
        match value? {
            "current" => Ok(AccountType::Current),
            "debit" => Ok(AccountType::Debit),
            _ => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "invalid account type"
                ))
            )
        }
    }
}

impl Type<Postgres> for AccountType {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("VARCHAR")
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "debug_derive", derive(Debug))]
pub struct Transfer {
    pub amount: Decimal,
    pub recipient: String,
}

#[derive(Clone)]
#[cfg_attr(feature = "debug_derive", derive(Debug))]
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

#[derive(Clone)]
#[cfg_attr(feature = "debug_derive", derive(Debug))]
pub struct Client {
    pub name: String,
    pub username: String,
    pub ci: i32,
    pub account_number: i32,
    pub balance: Decimal,
    pub account_type: AccountType,
    pub last_transaction: Option<Transaction>,
    pub suspended: bool,
    pub password_hash: String,
}

impl Client {
    pub fn iter(&self) -> ClientIterator {
        ClientIterator {
            client: self,
            index: 0,
        }
    }

    pub fn skip(&self, skip: usize) -> ClientIterator {
        ClientIterator {
            client: self,
            index: skip,
        }
    }

    pub async fn update_transaction(&mut self, pool: &PgPool) -> Result<()> {
        let client_row = sqlx::query("SELECT * FROM clients WHERE username = $1")
            .bind(&self.username)
            .fetch_optional(pool)
            .await?;

        if let Some(row) = client_row {
            let last_transaction: Option<String> = row.try_get("last_transaction")?;
            if last_transaction.is_some() {
                let transaction_row = sqlx::query("SELECT * FROM transactions WHERE username = $1")
                .bind(&self.username)
                .fetch_one(pool)
                .await?;
            
                self.last_transaction = Some(Transaction::from_row(&transaction_row)?);
            }
        }
        Ok(())
    }
}

pub struct ClientIterator<'a> {
    client: &'a Client,
    index: usize,
}

impl<'a> Iterator for ClientIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        match self.index - 1 {
            0 => Some(self.client.name.clone()),
            1 => Some(self.client.username.clone()),
            2 => Some(self.client.ci.to_string()),
            3 => Some(self.client.account_number.to_string()),
            4 => Some(self.client.balance.to_string()),
            5 => {
                match self.client.account_type {
                    AccountType::Current => Some("current".to_string()),
                    AccountType::Debit => Some("debit".to_string()),
                }
            }
            6 => Some("none".to_string()),
            7 => {
                if self.client.suspended { Some("suspended".to_string()) }
                else { Some( "not suspended".to_string()) }
            }
            _ => None
        }
    }
}

impl<'r> FromRow<'r, PgRow> for Client {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Client {
            account_number: row.try_get("account_number")?,
            username: row.try_get("username")?,
            password_hash: row.try_get("password")?,
            name: row.try_get("name")?,
            ci: row.try_get("ci")?,
            balance: row.try_get("balance")?,
            account_type: row.try_get("account_type")?,
            last_transaction: None,
            suspended: row.try_get("suspended")?,
        })
    }
}

pub struct ClientData {
    pub active: Option<Client>,
    pub actions: Vec<&'static str>,
    pub action_list_state: ListState,
    pub popups: HashMap<usize, Popup>,
}

impl std::default::Default for ClientData {
    fn default() -> Self {
        ClientData {
            active: None,
            actions: vec![
                "View info",
                "Make a deposit",
                "Make a withdrawal",
                "Make a transfer",
                "Change password"
            ],
            action_list_state: ListState::default(),
            popups: HashMap::from([
                (0, Popup::ViewInfo),
                (1, Popup::Deposit),
                (2, Popup::Withdraw),
                (3, Popup::Transfer),
                (4, Popup::ChangePsswd)
            ])
        }
    }
}