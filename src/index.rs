use crate::errors::{Result, ServerError};
use actix_web::{
    get,
    web::{Data, ServiceConfig},
    HttpResponse, Responder,
};

use aws_sdk_s3::Client as S3Client;

#[derive(serde::Serialize)]
struct Version {
    version: String,
    commit: String,
}

#[get("/health")]
async fn health(s3_client: Option<Data<S3Client>>) -> Result<HttpResponse> {
    let mut issues = vec![];

    if let Some(s3) = s3_client {
        if let Err(e) = s3.list_buckets().send().await {
            issues.push(format!("Failed to connect to S3 bucket: {}", e));
        }
    } else {
        issues.push("s3 client is not initialized".to_string());
    };

    if issues.is_empty() {
        Ok(HttpResponse::Ok().body("OK"))
    } else {
        Err(ServerError::HealthCheck { errors: issues })
    }
}

#[get("/version")]
async fn version() -> impl Responder {
    let version = Version {
        version: env!("CARGO_PKG_VERSION").to_string(),
        commit: option_env!("GH_SHA_REF")
            .unwrap_or("not_commit")
            .to_string(),
    };
    HttpResponse::Ok().json(version)
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is an API for personal use.")
}

pub fn index_config(cfg: &mut ServiceConfig) {
    cfg.service(index).service(version).service(health);
}
