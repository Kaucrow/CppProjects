// src/routes/users/logout.rs
use actix_web::{HttpResponse, web};
use sqlx::{PgPool, Row};

#[derive(serde::Serialize)]
struct UserValidation {
    is_superuser: bool,
}

impl TryFrom<sqlx::postgres::PgRow> for UserValidation {
    type Error = sqlx::Error;

    fn try_from(row: sqlx::postgres::PgRow) -> Result<Self, Self::Error> {
        Ok(UserValidation {
            is_superuser: row.try_get("is_superuser")?,
        })
    }
}

#[tracing::instrument(name = "Validate user", skip(session, pool))]
#[actix_web::get("/validate")]
pub async fn validate_user(
    session: actix_session::Session,
    pool: web::Data<PgPool>
) -> HttpResponse {
    match get_user_validation(&session, &pool).await {
        Ok(user_validation) => HttpResponse::Ok().json(user_validation),
        Err(_) => HttpResponse::Unauthorized().finish()
    }  
}

#[tracing::instrument(name = "Get user validation from user_id", skip(session, pool))]
pub async fn get_user_validation(
    session: &actix_session::Session,
    pool: &web::Data<PgPool>
) -> Result<UserValidation, String> {
    let user_id = match session_user_id(&session).await {
        Ok(user_id) => {
            tracing::debug!(target: "backend", "User id retrieved from the session.");
            user_id
        }
        Err(e) => {
            tracing::error!(target:"backend", "Failed to get user from session: {:#?}", e);
            return Err("We currently have some issues. Kindly try again and ensure you are logged in".to_string());
        }
    };

    let row = match sqlx::query("SELECT is_superuser FROM users WHERE id=$1")
        .bind(user_id)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => row,
        Err(_) => {
            tracing::error!(target: "backend", "Could not find a row with user id `{user_id}`");
            return Err(String::new());
        }
    };

    if let Ok(user_validation) = UserValidation::try_from(row) {
        Ok(user_validation)
    } else {
        Err(String::new())
    }
}

#[tracing::instrument(name = "Get user_id from session.", skip(session))]
pub async fn session_user_id(session: &actix_session::Session) -> Result<uuid::Uuid, String> {
    match session.get(crate::types::USER_ID_KEY) {
        Ok(user_id) => {
            tracing::debug!(target: "backend", "user_id: {:#?}", user_id);
            match user_id {
            Some(id) => Ok(id),
            None => Err("You are not authenticated".to_string()),
            }
        },
        Err(e) => Err(format!("{e}")),
    }
}