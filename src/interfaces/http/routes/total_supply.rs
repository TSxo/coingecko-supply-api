use actix_web::HttpResponse;
use actix_web::web::Data;
use serde::Serialize;
use tracing::{error, instrument};

use crate::domain::repositories::TokenSupplyRepository;
use crate::infrastructure::repositories::InMemoryTokenSupplyRepository;

#[derive(Serialize)]
struct Body {
    result: String,
}

/// Retrieves the total supply of tokens.
///
/// This endpoint fetches the total supply from the repository and returns it
/// as a JSON response.
///
/// # Arguments
///
/// * `state` - The repository containing token supply data.
#[instrument(skip(state))]
pub async fn total_supply(state: Data<InMemoryTokenSupplyRepository>) -> HttpResponse {
    match state.get_current().await {
        Ok(x) => {
            let b = Body {
                result: x.total_supply,
            };

            HttpResponse::Ok().json(b)
        }
        Err(e) => {
            error!("Failed to return total supply: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
