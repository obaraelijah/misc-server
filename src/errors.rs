use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug, Error)]
pub enum ServerError {
    #[error("Failed to retrieve objects: {message}")]
    ListObjects { message: String },
}

pub type Result<T> = std::result::Result<T, ServerError>;

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            ServerError::ListObjects { .. } => {
                actix_web::HttpResponse::InternalServerError().json(self)
            }
        }
    }
}