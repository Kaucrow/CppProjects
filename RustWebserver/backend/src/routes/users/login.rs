use sqlx::Row;

const USER_NOT_FOUND_MSG: &'static str = "A user with these details does not exist. If you registered with these details, ensure you activate your account by clicking on the link sent to your e-mail address.";

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct LoginUser {
    email: String,
    password: String,
}

#[tracing::instrument(name = "Logging a user in", skip( pool, user, session), fields(user_email = %user.email))]
#[actix_web::get("/login/")]
async fn login_user(
    pool: actix_web::web::Data<sqlx::postgres::PgPool>,
    user: actix_web::web::Json<LoginUser>,
    session: actix_session::Session,
) -> actix_web::HttpResponse {
    tracing::event!(target: "backend", tracing::Level::INFO, "Accesing LOGIN.");
    match get_user_who_is_active(&pool, &user.email).await {
        Ok(loggedin_user) => {
            let password_hash = loggedin_user.password.clone();
            let password = user.password.clone();

            let verify_result = tokio::task::spawn_blocking(move || {
                crate::utils::verify_password(password_hash, password)
            })
            .await
            .expect("Unable to unwrap JoinError.");

            match verify_result.await {
                Ok(()) => {
                    tracing::event!(target: "backend", tracing::Level::INFO, "User logged in successfully.");
                    session.renew();
                    session
                        .insert(crate::types::USER_ID_KEY, loggedin_user.id)
                        .expect("`user_id` cannot be inserted into session");
                    session
                        .insert(crate::types::USER_EMAIL_KEY, &loggedin_user.email)
                        .expect("`user_email` cannot be inserted into session");
                    
                    actix_web::HttpResponse::Ok().json(crate::types::UserVisible {
                        id: loggedin_user.id,
                        email: loggedin_user.email,
                        first_name: loggedin_user.first_name,
                        last_name: loggedin_user.last_name,
                        is_active: loggedin_user.is_active,
                        is_staff: loggedin_user.is_staff,
                        is_superuser: loggedin_user.is_superuser,
                        date_joined: loggedin_user.date_joined,
                        thumbnail: loggedin_user.thumbnail,
                    })
                }
                Err(e) => {
                    tracing::event!(target: "backend", tracing::Level::ERROR, "Wrong password: {:#?}", e);
                    actix_web::HttpResponse::NotFound().json(crate::types::ErrorResponse {
                        error: USER_NOT_FOUND_MSG.to_string()
                    })
                }
            }
        }
        Err(e) => {
            tracing::event!(target: "backend", tracing::Level::ERROR, "User not found: {:#?}", e);
            actix_web::HttpResponse::NotFound().json(crate::types::ErrorResponse {
                    error: USER_NOT_FOUND_MSG.to_string()
            })
        }
    }
}

#[tracing::instrument(name = "Getting a user from DB.", skip(pool, email),fields(user_email = %email))]
pub async fn get_user_who_is_active(
    pool: &sqlx::postgres::PgPool,
    email: &String,
) -> Result<crate::types::User, sqlx::Error> {
    match sqlx::query("SELECT id, email, password, first_name, last_name, is_staff, is_superuser, thumbnail, date_joined FROM users WHERE email = $1 AND is_active = TRUE")
        .bind(email)
        .map(|row: sqlx::postgres::PgRow| crate::types::User {
            id: row.get("id"),
            email: row.get("email"),
            password: row.get("password"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            is_active: true,
            is_staff: row.get("is_staff"),
            is_superuser: row.get("is_superuser"),
            thumbnail: row.get("thumbnail"),
            date_joined: row.get("date_joined"),
        })
        .fetch_one(pool)
        .await
    {
        Ok(user) => Ok(user),
        Err(e) => {
            tracing::event!(target: "sqlx",tracing::Level::ERROR, "User not found in DB: {:#?}", e);
            Err(e)
        }
    }
}