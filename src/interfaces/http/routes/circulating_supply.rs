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

/// Retrieves the current circulating supply of tokens.
///
/// This endpoint fetches the most up-to-date circulating supply from the
/// repository and returns it as a JSON response.
///
/// # Arguments
///
/// * `state` - The repository containing token supply data.
#[instrument(skip(state))]
pub async fn circulating_supply(state: Data<InMemoryTokenSupplyRepository>) -> HttpResponse {
    match state.get_current().await {
        Ok(x) => {
            let b = Body {
                result: x.circulating_supply,
            };

            HttpResponse::Ok().json(b)
        }
        Err(e) => {
            error!("Failed to return circulating supply: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
