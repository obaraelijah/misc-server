use actix_identity::{Identity, IdentityExt};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    get,
    http::StatusCode,
    post,
    web::{Json, ServiceConfig},
    Error, HttpMessage, HttpRequest, HttpResponse, HttpResponseBuilder, Responder,
};
use actix_web_lab::middleware::Next;
use serde::{Deserialize, Serialize};
use ldap3::{Ldap, LdapConnAsync};

use crate::errors::{Result, ServerError};

pub fn auth_config(cfg: &mut ServiceConfig) {
    cfg.service(user).service(login).service(logout);
}

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct User(String);

pub async fn create_ldap_conn(url: &str) -> Result<(LdapConnAsync, Ldap)> {
    // TODO: Connection server
    let (con, ldap) = LdapConnAsync::new(url)
        .await
        .map_err(|e| ServerError::Login {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Failed to connect to LDAP server: {}", e),
        })?;

    Ok((con, ldap))
}

#[get("/user")]
pub async fn user(id: Option<Identity>) -> Result<HttpResponse> {
    id.map(|u| HttpResponseBuilder::new(StatusCode::OK).json(User(u.id().unwrap())))
        .ok_or(ServerError::Login {
            code: StatusCode::UNAUTHORIZED,
            message: "Not logged in".to_string(),
        })
}

#[post("/login")]
pub async fn login(req: HttpRequest, login_details: Json<LoginRequest>) -> Result<HttpResponse> {
    let (con, mut ldap) = create_ldap_conn("ldap://localhost:3890").await?;

    ldap3::drive!(con);
    
    Identity::login(&req.extensions(), login_details.username.clone()).map_err(|e| {
        ServerError::Login {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!("Failed to save session: {}", e),
        }
    })?;

    Ok(HttpResponse::Ok().body("Logged in"))
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
