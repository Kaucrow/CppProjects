mod register;

use actix_web::web;

pub fn auth_routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(register::register_user)
    );
}