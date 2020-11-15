use crate::{routes::RouteSet, structs::HealthResponse};
use rocket::Route;
use rocket_contrib::json::Json;

pub struct MiscRoutes;

impl RouteSet for MiscRoutes {
    fn base(&self) -> String {
        "/".to_string()
    }
    fn routes(&self) -> Vec<Route> {
        routes![health]
    }
}

#[get("/health", format = "json")]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { healthy: true })
}
