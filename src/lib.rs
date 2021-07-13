use actix_web::{error, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VersionInfo {
    pub title: String,
    pub version: String,
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(VersionInfo {
        title: "qdrant - vector search engine".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}