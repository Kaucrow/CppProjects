use actix_web::middleware;
// src/startup.rs
use sqlx;
use deadpool_redis;
use std::{fs::File, io::BufReader};
use rustls::{pki_types::PrivateKeyDer, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};

pub struct Application {
    port: u16,
    server: actix_web::dev::Server,
}

impl Application {
    pub async fn build(
        settings: crate::settings::Settings,
        test_pool: Option<sqlx::postgres::PgPool>,
    ) -> Result<Self, std::io::Error> {
        let connection_pool = if let Some(pool) = test_pool {
            pool
        } else if settings.debug {
            get_connection_pool(&settings.database).await
        } else {
            let db_url = std::env::var("DATABASE_URL").expect("Failed to get DATABASE_URL.");
            match sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&db_url)
                .await
            {
                Ok(pool) => pool,
                Err(e) => {
                    tracing::event!(target: "sqlx", tracing::Level::ERROR, "Couldn't establish DB conn.: {:#?}", e);
                    panic!("Couldn't establish DB connection!")
                }
            }
        };

        sqlx::migrate!()
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database.");

        let port = settings.application.port;
        let server = run(connection_pool, settings).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_connection_pool(
    settings: &crate::settings::DatabaseSettings,
) -> sqlx::postgres::PgPool {
    sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(settings.connect_to_db())
}

async fn run(
    db_pool: sqlx::postgres::PgPool,
    settings: crate::settings::Settings,
) -> Result<actix_web::dev::Server, std::io::Error> {
    // Database connection pool application state
    let pool = actix_web::web::Data::new(db_pool);

    // Redis connection pool
    let cfg = deadpool_redis::Config::from_url(settings.clone().redis.uri);
    let redis_pool = cfg
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Cannot create deadpool redis.");
    let redis_pool_data = actix_web::web::Data::new(redis_pool);

    // For session
    let secret_key = actix_web::cookie::Key::from(settings.secret.hmac_secret.as_bytes());

    let rustls_config = load_rustls_config();

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
            actix_cors::Cors::default()
                .allowed_origin(&settings.frontend_url)
                .allowed_origin("https://4c45-149-40-62-38.ngrok-free.app")
                .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE", "OPTIONS"])
                .allowed_headers(vec![
                    actix_web::http::header::AUTHORIZATION,
                    actix_web::http::header::ACCEPT,
                ])
                .allowed_header("ngrok-skip-browser-warning")
                .allowed_header(actix_web::http::header::CONTENT_TYPE)
                .expose_headers(&[actix_web::http::header::CONTENT_DISPOSITION])
                .supports_credentials()
                .max_age(3600),
            )
            .wrap(
                actix_session::SessionMiddleware::builder(
                    actix_session::storage::CookieSessionStore::default(),
                    secret_key.clone(),
                )
                .cookie_http_only(true)
                .cookie_same_site(actix_web::cookie::SameSite::Lax)
                .cookie_secure(true)
                .build()
            )
            .service(crate::routes::health_check)
            .service(crate::routes::get_num)
            .service(crate::routes::add_num)
            .configure(crate::routes::auth_routes_config)
            // Add database pool to application state
            .app_data(pool.clone())
            // Add redis pool to application state
            .app_data(redis_pool_data.clone())
            .wrap(middleware::NormalizePath::trim())
    });

    let server = if settings.application.protocol == "https" {
        server.bind_rustls_0_23(format!("{}:{}", settings.application.host, settings.application.port), rustls_config)?
        .run()
    } else {
        let address = format!(
            "{}:{}",
            settings.application.host, settings.application.port
        );
        let listener = std::net::TcpListener::bind(&address)?;
        server.listen(listener)?
        .run()
    };

    Ok(server)
}

fn load_rustls_config() -> rustls::ServerConfig {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("certificate/localhost+1.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("certificate/localhost+1-key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}