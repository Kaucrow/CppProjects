use argon2::password_hash::rand_core::{OsRng, RngCore};
use tracing_subscriber::fmt::time;
use core::convert::TryFrom;
use deadpool_redis::redis::AsyncCommands;
use hex;
use serde_json::json;
use pasetors::{
    claims::{Claims, ClaimsValidationRules},
    keys::SymmetricKey,
    token::UntrustedToken,
    local,
    version4::V4,
    Local
};

/// Store the session key prefix as a const so it can't be typo'd anywhere it's used.
const SESSION_KEY_PREFIX: &str = "valid_session_key_for_{}";

/// Issues a pasetor token to a user. The token has the user's id encoded.
/// A session_key is also encoded. This key is used to destroy the token
/// as soon as it's been verified. Depending on its usage, the token issued
/// has at most an hour to live. Which means, it is destroyed after its time-to-live.
#[tracing::instrument(name = "Issue pasetors token", skip(redis_connection))]
pub async fn issue_confirmation_token_pasetors(
    user_id: uuid::Uuid,
    redis_connection: &mut deadpool_redis::redis::aio::MultiplexedConnection,
    is_for_password_change: Option<bool>,
) -> Result<String, deadpool_redis::redis::RedisError> {
    // I just generate 128 bytes of random data for the session key
    // from something that is cryptographically secure (rand::CryptoRng)
    //
    // You don't necessarily need a random value, but you'll want something
    // that is sufficiently not able to be guessed (you don't want someone getting
    // an old token that is supposed to not be live, and being able to get a valid
    // token from that). 
    let session_key: String = {
        let mut buf = [0_u8; 128];
        OsRng.fill_bytes(&mut buf);
        hex::encode(buf)
    };

    let redis_key = {
        if is_for_password_change.is_some() {
            format!(
                "{}{}is_for_password_change",
                SESSION_KEY_PREFIX, session_key
            )
        } else {
            format!("{}{}", SESSION_KEY_PREFIX, session_key)
        }
    };

    redis_connection
        .set(
            redis_key.clone(),
            // since we only validate that the key exists
            // to indicate the session is "live", it can have any value
            String::new(),
        )
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "RedisError (set): {}", e);
            e
        })?;

    let settings = crate::settings::get_settings().expect("Cannot load settings.");
    let current_date_time = chrono::Local::now();
    // for redis expiration
    let time_to_live = {
        if is_for_password_change.is_some() {
            chrono::Duration::hours(1)
        } else {
            chrono::Duration::minutes(settings.secret.token_expiration)
        }
    };
    // for claims expiration
    let dt = current_date_time + time_to_live;

    redis_connection
        .expire(
            redis_key.clone(),
            time_to_live.num_seconds().try_into().unwrap()
        )
        .await
        .map_err(|e| {
            tracing::event!(target: "backend", tracing::Level::ERROR, "RedisError (expiry): {}", e);
            e
        })?;

    let mut claims = Claims::new().unwrap();
    // set custom expiration, default is 1 hour
    claims.expiration(&dt.to_rfc3339()).unwrap();
    claims
        .add_additional("user_id", json!(user_id))
        .unwrap();
    claims
        .add_additional("session_key", json!(session_key))
        .unwrap();

    // use the 256 bit secret key as the symmetric key 
    let sk = SymmetricKey::<V4>::from(settings.secret.secret_key.as_bytes()).unwrap();

    Ok(local::encrypt(
        &sk,
        &claims,
        None,
        Some(settings.secret.hmac_secret.as_bytes()),
    )
    .unwrap())
}