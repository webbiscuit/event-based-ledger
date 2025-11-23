pub mod routes;

mod health_handler;
mod new_account_handler;
mod get_account_events_handler;
mod deposit_handler;
mod balance_handler;
mod withdrawal_handler;

pub use routes::create_router;