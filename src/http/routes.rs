use std::net::SocketAddr;

use axum::{
    Router, routing::{get, post}
};

use crate::{AppState, http::{get_account_events_handler::get_account_events_handler, health_handler::health_handler, new_account_handler::new_account_handler}};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .route("/accounts", post(new_account_handler))
        .route("/accounts/{account_id}/events", get(get_account_events_handler))
        .with_state(state)
}

pub fn format_listen_addr(port: u16) -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], port))
}