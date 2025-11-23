use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct NewAccountResponse {
    status: &'static str,
}

pub async fn new_account_handler() -> Json<NewAccountResponse> {
    Json(NewAccountResponse { status: "Great!" })
}