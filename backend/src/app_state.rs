use crate::payment::Payment;

pub struct AppState{
    pub payment: Payment
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            payment: Payment::new()
        }
    }
}