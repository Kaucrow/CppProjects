use serde::Deserialize;
use actix_web::{
    HttpResponse,
    http,
    web,
};
use sqlx::postgres::PgPool;

#[derive(Deserialize)]
pub struct Parameters {
    token: String,
}

#[tracing::instrument(
    name = "Activating a new user",
    skip(parameters, pool, redis_pool)
)]
#[actix_web::get("/register/confirm")]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: web::Data<PgPool>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");

    let mut redis_con = redis_pool
        .get()
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{}", e);

            // if the account cannot be activated (due to redis connection failed)
            HttpResponse::SeeOther()
                .insert_header((
                    http::header::LOCATION,
                    format!("{}/auth/error", settings.frontend_url),
                ))
                .finish()
        })
        .expect("Redis connection cannot be obtained.");

    let confirmation_token = match crate::utils::verify_confirmation_token_pasetors(
        parameters.token.clone(),
        &mut redis_con,
        None
    )
    .await
    {
        Ok(token) => token,
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{:#?}", e);

            // if the token has expired or has already been used
            return HttpResponse::SeeOther().insert_header((
                    http::header::LOCATION,
                    format!("{}/auth/regenerate-token", settings.frontend_url)
                ))
                .finish();
        }
    };

    match activate_new_user(&pool, confirmation_token.user_id).await {
        Ok(_) => {
            tracing::event!(target: "backend", tracing::Level::INFO, "New user was activated successfully.");

            // if the user is activated successfully
            HttpResponse::SeeOther()
                .insert_header((
                    http::header::LOCATION,
                    format!("{}/auth/confirmed", settings.frontend_url),
                ))
                .finish()
        }
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Cannot activate account: {}", e);

            // if the account cannot be activated
            HttpResponse::SeeOther()
                .insert_header((
                    http::header::LOCATION,
                    format!("{}/auth/error?reason={e}", settings.frontend_url),
                ))
                .finish()
        }
    }
}

#[tracing::instrument(name = "Mark a user active", skip(pool), fields(
    new_user_user_id = %user_id
))]
pub async fn activate_new_user(
    pool: &PgPool,
    user_id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    match sqlx::query("UPDATE users SET is_active=true WHERE id=$1")
        .bind(user_id)
        .execute(pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("Failed to execute query: {:#?}", e);
            Err(e)
        }
    }
}