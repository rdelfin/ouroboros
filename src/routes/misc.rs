use crate::structs::HealthResponse;
use rocket_contrib::json::Json;

#[get("/health", format = "json")]
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { healthy: true })
}
