use actix_identity::{Identity, IdentityExt};
use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, get, http::StatusCode, post, web::ServiceConfig, Error, HttpResponse, HttpResponseBuilder, Responder};
use actix_web_lab::middleware::Next;
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

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> std::result::Result<ServiceResponse<impl MessageBody>, Error> {
    if req.get_identity().is_err() {
        return Ok(req.into_response(
            HttpResponse::Unauthorized()
                .body("not logged in")
                .map_into_right_body(),
        ));
    }

    next.call(req)
        .await
        .map(ServiceResponse::map_into_left_body)
}