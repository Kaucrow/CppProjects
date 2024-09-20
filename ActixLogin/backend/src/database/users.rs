use sqlx::{Postgres, postgres::PgRow, Row};
use crate::types::users::NewUser;

#[tracing::instrument(
    name = "Inserting new user into DB.",
    skip(transaction, new_user),
    fields(
        new_user_email = %new_user.email,
        new_user_first_name = %new_user.first_name,
        new_user_last_name = %new_user.last_name
    )
)]
pub async fn insert_created_user_into_db(
    transaction: &mut sqlx::Transaction<'_, Postgres>,
    new_user: &NewUser
) -> Result<uuid::Uuid, sqlx::Error> {
    let user_id = match sqlx::query(
        "INSERT INTO users (email, password, first_name, last_name) VALUES ($1, $2, $3, $4) RETURNING id",
    )
    .bind(&new_user.email)
    .bind(&new_user.password)
    .bind(&new_user.first_name)
    .bind(&new_user.last_name)
    .map(|row: PgRow| -> uuid::Uuid{
        row.get("id")
    })
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "sqlx", tracing::Level::ERROR, "Failed to insert user into DB: {:#?}", e);
            return Err(e);
        }
    };

    match sqlx::query(
        "INSERT INTO user_profile (user_id) VALUES ($1)
            ON CONFLICT (user_id)
            DO NOTHING",
    )
    .bind(user_id)
    .execute(&mut **transaction)
    .await
    {
        Ok(_) => {
            tracing::event!(target: "sqlx", tracing::Level::INFO, "User profile created sucessfully {}.", user_id);
            Ok(user_id)
        }
        Err(e) => {
            tracing::event!(target: "sqlx", tracing::Level::ERROR, "Failed to insert user's profile into DB: {:#?}", e);
            Err(e)
        }
    }
}