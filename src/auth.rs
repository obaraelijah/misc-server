use actix_identity::{Identity, IdentityExt};
use actix_web::{get, http::StatusCode, post, web::ServiceConfig, HttpResponse, HttpResponseBuilder, Responder};
use serde::{Deserialize, Serialize};

use crate::errors::{Result, ServerError};

pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.service(user).service(logout);
}

#[derive(Serialize, Debug)]
pub struct User(String);

#[get("/user")]
pub async fn user(id: Option<Identity>) -> Result<HttpResponse> {
    id.map(|u| HttpResponseBuilder::new(StatusCode::OK).json(User(u.id().unwrap())))
        .ok_or(ServerError::Login {
            code: StatusCode::UNAUTHORIZED,
            message: "Not logged in".to_string(),
        })
}

#[post("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.logout();

    HttpResponse::Ok()
}