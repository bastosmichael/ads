use actix_web::web::Path;
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
use crate::response::Response;

pub type Clicks = Response<Click>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Click {
    pub id: String,
    pub created_at: DateTime<Utc>,
}

impl Click {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
        }
    }
}

/// list last 50 clicks from a ad `/ads/{id}/clicks`
#[get("/ads/{id}/clicks")]
pub async fn list(path: Path<(String,)>) -> HttpResponse {
    // TODO find clicks by ad ID and return them
    let clicks = Clicks { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(clicks)
}

/// add one click to a ad `/ads/{id}/clicks`
#[post("/ads/{id}/clicks")]
pub async fn redirect(path: Path<(String,)>) -> HttpResponse {
    // TODO add one click to a ad
    let click = Click::new();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(click)
}

/// remove one click from a ad `/ads/{id}/clicks`
#[delete("/ads/{id}/clicks")]
pub async fn reject(path: Path<(String,)>) -> HttpResponse {
    // TODO remove one click to a ad
    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
