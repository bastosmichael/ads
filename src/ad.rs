use actix_web::web::{Json, Path};
use actix_web::HttpResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
use crate::click::Click;
use crate::response::Response;

pub type Ads = Response<Ad>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Ad {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub url: String,
    pub clicks: Vec<Click>,
}

impl Ad {
    pub fn new(url: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            created_at: Utc::now(),
            url,
            clicks: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdRequest {
    pub url: Option<String>,
}

impl AdRequest {
    pub fn to_ad(&self) -> Option<Ad> {
        match &self.url {
            Some(url) => Some(Ad::new(url.to_string())),
            None => None,
        }
    }
}

/// list 50 last ads `/ads`
#[get("/ads")]
pub async fn list() -> HttpResponse {
    // TODO find the last 50 ads and return them

    let ads = Ads { results: vec![] };

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(ads)
}

/// create a ad `/ads`
#[post("/ads")]
pub async fn create(ad_req: Json<AdRequest>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(ad_req.to_ad())
}

/// find a ad by its id `/ads/{id}`
#[get("/ads/{id}")]
pub async fn get(path: Path<(String,)>) -> HttpResponse {
    // TODO find ad a ad by ID and return it
    let found_ad: Option<Ad> = None;

    match found_ad {
        Some(ad) => HttpResponse::Ok()
            .content_type(APPLICATION_JSON)
            .json(ad),
        None => HttpResponse::NoContent()
            .content_type(APPLICATION_JSON)
            .await
            .unwrap(),
    }
}

/// delete a ad by its id `/ads/{id}`
#[delete("/ads/{id}")]
pub async fn delete(path: Path<(String,)>) -> HttpResponse {
    // TODO delete ad by ID
    // in any case return status 204

    HttpResponse::NoContent()
        .content_type(APPLICATION_JSON)
        .await
        .unwrap()
}
