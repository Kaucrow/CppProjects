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

#[derive(Debug)]
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

#[cfg_attr(feature = "debug_derive", derive(Debug))]
pub struct Transfer {
    pub amount: Decimal,
    pub recipient: String,
}

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

#[cfg_attr(feature = "debug_derive", derive(Debug))]
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
    pub fn new() -> Self {
        Client {
            account_number: 0,
            username: String::new(),
            name: String::new(),
            ci: 0,
            account_type: AccountType::Current,
            balance: Decimal::new(0, 2),
            last_transaction: None,
            suspended: false,
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

impl<'r> FromRow<'r, PgRow> for Client {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Client {
            account_number: row.try_get("account_number")?,
            username: row.try_get("username")?,
            name: row.try_get("name")?,
            ci: row.try_get("ci")?,
            balance: row.try_get("balance")?,
            account_type: row.try_get("account_type")?,
            last_transaction: None,
            suspended: row.try_get("suspended")?,
        })
    }
}