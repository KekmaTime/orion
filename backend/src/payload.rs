use serde::Serialize;

#[derive(Serialize)]
pub struct PaymentSessionResponse {
    pub url: String,
}