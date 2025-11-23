use axum::{Json, extract::{Path, State}, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::{AppState, domain::{Currency, Money, errors::DomainError, types::AccountId}};

#[derive(Deserialize)]
pub struct WithdrawalRequest {
    amount_minor: i64,
    currency: String,
}

#[derive(Serialize)]
pub struct WithdrawalResponse {
    id: uuid::Uuid,
    account_id: AccountId,
    amount_minor: i64,
    currency: String,
}

pub async fn withdrawal_handler(
    State(state): State<AppState>,
    Path(account_id): Path<String>,
    Json(body): Json<WithdrawalRequest>,
) -> Result<(StatusCode, Json<WithdrawalResponse>), (StatusCode, String)> {
    let account_uuid = 
        account_id.parse().map_err(|_| (StatusCode::BAD_REQUEST, "Invalid account id".to_string()))?;

    let currency = match body.currency.as_str() {
        "GBP" => Currency::Gbp,
        other => {
            return Err((StatusCode::BAD_REQUEST, format!("Unsupported currency: {other}")))
        }
    };

    let money = Money::new_minor(body.amount_minor, currency)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid amount {e}")))?;

    let mut ledger_guard = 
        state.ledger.lock().map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Ledger unavailable".to_string()))?;

    let id = ledger_guard.withdraw(account_uuid, money)
        .map_err(|err| match err {
            DomainError::AccountNotFound => {
                (StatusCode::NOT_FOUND, "Account not found".to_string())
            }
            DomainError::InsufficientFunds { .. } => {
                (StatusCode::BAD_REQUEST, "Insufficient funds".to_string())
            }
            _ =>(StatusCode::INTERNAL_SERVER_ERROR, "Ledger error".to_string())
        })?;

    let response = WithdrawalResponse {
        id,
        account_id: account_uuid,
        amount_minor: body.amount_minor,
        currency: body.currency,
    };
    
    Ok((StatusCode::CREATED, Json(response)))
}