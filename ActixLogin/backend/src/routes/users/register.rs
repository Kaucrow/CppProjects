use sqlx::postgres::PgPool;
use actix_web::{web, HttpResponse};
use crate::{
    utils::{
        auth::password::hash,
        send_multipart_email,
    },
    database::insert_created_user_into_db,
    types::{
        NewUser,
        SuccessResponse,
        ErrorResponse,
    },
};

#[tracing::instrument(name = "Adding a new user",
skip(pool, new_user, redis_pool),
fields(
    new_user_email = %new_user.email,
    new_user_first_name = %new_user.first_name,
    new_user_last_name = %new_user.last_name,
))]
#[actix_web::post("/register")]
pub async fn register_user(
    new_user: web::Json<NewUser>,
    pool: web::Data<PgPool>,
    redis_pool: web::Data<deadpool_redis::Pool>,
) -> HttpResponse {
    tracing::event!(target: "backend", tracing::Level::INFO, "Reached /users/register");
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "Unable to begin DB transaction: {:#?}", e);
            return HttpResponse::InternalServerError().json(
                ErrorResponse {
                    error: "Something unexpected happened. Please try again.".to_string(),
                },
            );
        }
    };

    let hashed_password = hash(new_user.0.password.as_bytes()).await;

    let create_new_user = NewUser {
        password: hashed_password,
        email: new_user.0.email,
        first_name: new_user.0.first_name,
        last_name: new_user.0.last_name,
    };

    let user_id = match insert_created_user_into_db(&mut transaction, &create_new_user).await {
        Ok(id) => id,
        Err(e) => {
            tracing::event!(target: "sqlx", tracing::Level::ERROR, "Failed to insert user into DB: {:#?}", e);
            // if e is of type "Violation of unique index or unique constraint"
            let error_message = if e
                .as_database_error()
                .unwrap()
                .code()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                == 23505
            {
                ErrorResponse {
                    error: "A user with that email already exists.".to_string(),
                }
            } else {
                ErrorResponse {
                    error: "Error inserting user into the database".to_string(),
                }
            };
            return HttpResponse::InternalServerError().json(error_message);
        }
    };

    // open redis connection
    let mut redis_con = redis_pool
        .get()
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "{}", e);
            actix_web::HttpResponse::InternalServerError().json(ErrorResponse {
                error: "We cannot activate your account at the moment".to_string(),
            })
        })
        .expect("Redis connection cannot be obtained.");

    send_multipart_email(
        "Actix Login Sign Up".to_string(),
        user_id,
        create_new_user.email,
        create_new_user.first_name,
        create_new_user.last_name,
        "verification_email.html",
        &mut redis_con
    )
    .await
    .unwrap();

    if transaction.commit().await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    tracing::event!(target: "backend", tracing::Level::INFO, "User created successfully.");
    actix_web::HttpResponse::Ok().json(SuccessResponse {
        message: "Your account was created successfully. Check your email address to activate your account as we just sent you an activation link. Ensure you activate your account before the link expires".to_string(),
    })
}