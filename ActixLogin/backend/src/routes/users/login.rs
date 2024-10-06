use sqlx::{postgres::PgPool, Row};
use actix_web::{web, HttpResponse};
use crate::types::LoginUser;

const USER_NOT_FOUND_MSG: &'static str = "A user with these details does not exist. If you registered with these details, ensure you activated your account by clicking on the link sent to your e-mail address.";

#[tracing::instrument(name = "Logging a user in", skip(pool, user, session), fields(user_email = %user.email))]
#[actix_web::post("/login")]
async fn login_user(
    user: web::Json<LoginUser>,
    pool: web::Data<PgPool>,
    session: actix_session::Session,
) -> HttpResponse {
    tracing::info!(target: "backend", "Accessing LOGIN.");
    match get_user_who_is_active(&pool, &user.email).await {
        Ok(db_user) => {
            let password_hash = db_user.password.clone();
            let password = user.password.clone();

            let verify_result = tokio::task::spawn_blocking(move || {
                crate::utils::verify_password(password_hash, password)
            })
            .await
            .expect("Unable to unwrap JoinError.");

            match verify_result.await {
                Ok(()) => {
                    tracing::info!(target: "backend", "User logged in successfully.");
                    session.renew();
                    session
                        .insert(crate::types::USER_ID_KEY, db_user.id)
                        .expect("`user_id` cannot be inserted into session");
                    session
                        .insert(crate::types::USER_EMAIL_KEY, &db_user.email)
                        .expect("`user_email` cannot be inserted into session");

                    HttpResponse::Ok().json(crate::types::UserVisible {
                        id: db_user.id,
                        email: db_user.email,
                        first_name: db_user.first_name,
                        last_name: db_user.last_name,
                        is_active: db_user.is_active,
                        is_superuser: db_user.is_superuser,
                        date_joined: db_user.date_joined,
                        thumbnail: db_user.thumbnail,
                    })
                }
                Err(e) => {
                    tracing::event!(target: "backend", tracing::Level::ERROR, "Wrong password: {:#?}", e);
                    HttpResponse::NotFound().json(crate::types::ErrorResponse {
                        error: USER_NOT_FOUND_MSG.to_string()
                    })
                }
            }
        }
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "User not found: {:#?}", e);
            HttpResponse::NotFound().json(crate::types::ErrorResponse {
                error: USER_NOT_FOUND_MSG.to_string()
            })
        }
    }
}

#[tracing::instrument(name = "Getting a user from DB.", skip(pool, email),fields(user_email = %email))]
pub async fn get_user_who_is_active(
    pool: &PgPool,
    email: &String,
) -> Result<crate::types::User, sqlx::Error> {
    match sqlx::query("SELECT id, email, password, first_name, last_name, is_superuser, thumbnail, date_joined FROM users WHERE email = $1 AND is_active = TRUE")
        .bind(email)
        .map(|row: sqlx::postgres::PgRow| crate::types::User {
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_active: true,
            is_superuser: row.get("is_superuser"),
            thumbnail: row.get("thumbnail"),
            date_joined: row.get("date_joined"),
        })
        .fetch_one(pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => {
            tracing::event!(target: "sqlx", tracing::Level::ERROR, "User not found in DB: {:#?}", e);
            Err(e)
        }
    }
}