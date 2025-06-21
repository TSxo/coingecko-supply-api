use actix_web::HttpResponse;
use actix_web::web::Data;
use tracing::{error, instrument};

use crate::application::port::inbound::TokenSupplyService;
use crate::infrastructure::adapter::inbound::http::dto::SupplyResponse;

/// Retrieves the current circulating supply of tokens.
///
/// This endpoint fetches the most up-to-date circulating supply from the
/// repository and returns it as a JSON response.
///
/// # Arguments
///
/// * `state` - The repository containing token supply data.
#[instrument(skip(state))]
pub async fn circulating_supply(state: Data<impl TokenSupplyService>) -> HttpResponse {
    match state.get_token_supply().await {
        Ok(x) => {
            let b = SupplyResponse::new(x.circulating_supply);
            HttpResponse::Ok().json(b)
        }
        Err(e) => {
            error!("Failed to return circulating supply: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
