use actix_web::web;

pub mod health;
pub mod auth;
pub mod products;
pub mod orders;
pub mod customers;
pub mod inventory;
pub mod suppliers;
pub mod webhooks;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/health")
                .configure(health::configure)
        )
        .service(
            web::scope("/api/v1")
                .configure(auth::configure)
                .configure(products::configure)
                .configure(orders::configure)
                .configure(customers::configure)
                .configure(inventory::configure)
                .configure(suppliers::configure)
        )
        .service(
            web::scope("/webhooks")
                .configure(webhooks::configure)
        );
}
