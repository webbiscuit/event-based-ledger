use axum::{Json, extract::{Path, State}, http::StatusCode};
use serde::Serialize;

use crate::{AppState, domain::{errors::DomainError, events::LedgerEvent, types::AccountId}};

#[derive(Serialize)]
pub struct AccountEventsResponse {
    id: AccountId
}

pub async fn get_account_events_handler(
    State(state): State<AppState>,
    Path(account_id): Path<String>
) -> Result<Json<Vec<LedgerEvent>>, (StatusCode, String)> {
    let account_uuid = 
        account_id.parse().map_err(|_| (StatusCode::BAD_REQUEST, "Invalid account id".to_string()))?;

    let ledger_guard = 
        state.ledger.lock().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Ledger unavailable".to_string()))?;

    let events = ledger_guard.events_for_account(account_uuid)
        .map_err(|err|  match err {
            DomainError::AccountNotFound => (StatusCode::NOT_FOUND, "Account not found".to_string()),
            _ =>(StatusCode::INTERNAL_SERVER_ERROR, "Error fetching events".to_string())
        })?;

    Ok(Json(events))
}