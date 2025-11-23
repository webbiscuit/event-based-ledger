use axum::{Json, extract::{Path, State}, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{AppState, domain::{Currency, Money, errors::DomainError, types::AccountId}};

#[derive(Serialize)]
pub struct BalanceResponse {
    account_id: AccountId,
    amount_minor: i64,
    currency: String,
    display: String,
}

pub async fn balance_handler(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
) -> Result<(StatusCode, Json<BalanceResponse>), (StatusCode, String)> {
    let account_uuid = 
        account_id.parse().map_err(|_| (StatusCode::BAD_REQUEST, "Invalid account id".to_string()))?;

    let ledger_guard = 
        state.ledger.lock().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Ledger unavailable".to_string()))?;

    let balance = ledger_guard.balance_for_account(account_uuid).map_err(
        |_| (StatusCode::INTERNAL_SERVER_ERROR, "Balance unavailable".to_string())
    )?;

    let response = BalanceResponse {
        account_id: account_uuid,
        amount_minor: balance.amount(),
        currency: balance.currency().code().to_string(),
        display: balance.to_string()
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}