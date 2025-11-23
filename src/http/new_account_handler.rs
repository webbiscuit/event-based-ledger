use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::{AppState, domain::types::AccountId};

#[derive(Serialize)]
pub struct NewAccountResponse {
    id: AccountId
}

pub async fn new_account_handler(
    State(state): State<AppState>
) -> (StatusCode, Json<NewAccountResponse>) {
    let mut ledger_guard = 
        state.ledger.lock().expect("Mutex poisoned");

    let account_id = ledger_guard.open_account();

    (StatusCode::CREATED, Json(NewAccountResponse { id: account_id }))
}