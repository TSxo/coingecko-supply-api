use actix_web::HttpResponse;

/// Health check endpoint.
///
/// Provides a simple way to verify that the service is running and responding
/// to requests. Returns a 200 OK status with an empty body.
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
