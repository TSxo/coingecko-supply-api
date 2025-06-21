use serde::Serialize;

#[derive(Serialize)]
pub struct SupplyResponse {
    result: String,
}

impl SupplyResponse {
    pub fn new(result: String) -> Self {
        Self { result }
    }
}
