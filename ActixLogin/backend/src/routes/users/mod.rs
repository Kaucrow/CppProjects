mod register;
mod confirm_registration;
mod login;

use actix_web::web;

pub fn auth_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(register::register_user)
            .service(confirm_registration::confirm)
            .service(login::login_user)
    );
}