pub mod routes;

mod health_handler;
mod new_account_handler;
mod get_account_events_handler;
mod deposit_handler;

pub use routes::create_router;