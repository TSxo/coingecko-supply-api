use actix_web::HttpResponse;
use actix_web::web::Data;
use tracing::{error, instrument};

use crate::application::port::inbound::TokenSupplyService;
use crate::infrastructure::adapter::inbound::http::dto::SupplyResponse;

/// Retrieves the total supply of tokens.
///
/// This endpoint fetches the total supply from the repository and returns it
/// as a JSON response.
///
/// # Arguments
///
/// * `state` - The repository containing token supply data.
#[instrument(skip(state))]
pub async fn total_supply(state: Data<impl TokenSupplyService>) -> HttpResponse {
    match state.get_token_supply().await {
        Ok(x) => {
            let b = SupplyResponse::new(x.total_supply);
            HttpResponse::Ok().json(b)
        }
        Err(e) => {
            error!("Failed to return total supply: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
