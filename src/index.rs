use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

#[derive(serde::Serialize)]
struct Version {
    version: String,
    commit: String,
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
    cfg.service(index).service(version);
}
