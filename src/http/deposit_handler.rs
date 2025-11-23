use axum::{Json, extract::{Path, State}, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{AppState, domain::{Currency, Money, errors::DomainError, types::AccountId}};

#[derive(Deserialize)]
pub struct DepositRequest {
    amount_minor: i64,
    currency: String,
}

#[derive(Serialize)]
pub struct DepositResponse {
    id: uuid::Uuid,
    account_id: AccountId,
    amount_minor: i64,
    currency: String,
}

pub async fn deposit_handler(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
    Json(body): Json<DepositRequest>,
) -> Result<(StatusCode, Json<DepositResponse>), (StatusCode, String)> {
    let account_uuid = 
        account_id.parse().map_err(|_| (StatusCode::BAD_REQUEST, "Invalid account id".to_string()))?;

    let currency = match body.currency.as_str() {
        "GBP" => Currency::GBP,
        other => {
            return Err((StatusCode::BAD_REQUEST, format!("Unsupported currency: {}", other)))
        }
    };

    let money = Money::new_minor(body.amount_minor, currency)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid amount {e}").to_string()))?;

    let mut ledger_guard = 
        state.ledger.lock().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Ledger unavailable".to_string()))?;

    let id = ledger_guard.deposit(account_uuid, money)
        .map_err(|err| match err {
            DomainError::AccountNotFound => {
                (StatusCode::NOT_FOUND, "Account not found".to_string())
            }
            _ =>(StatusCode::INTERNAL_SERVER_ERROR, "Ledger error".to_string())
        })?;

    let response = DepositResponse {
        id,
        account_id: account_uuid,
        amount_minor: body.amount_minor,
        currency: body.currency,
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}