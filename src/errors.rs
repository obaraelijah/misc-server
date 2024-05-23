use aws_sdk_s3::{error::SdkError, operation::list_objects_v2::ListObjectsV2Error};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Debug, Error)]
pub enum ServerError {
    #[error("Failed to retrieve objects: {message}")]
    ListObjects { message: String },
}

impl From<SdkError<ListObjectsV2Error>>  for ServerError {
    fn from(e: SdkError<ListObjectsV2Error>) -> Self {
        ServerError::ListObjects { 
            message: e.to_string(),
        }
    }
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