use actix_web::{middleware::Logger, App, HttpServer};
use aws_credential_types::Credentials;
use env_logger::Env;
use index::index_config;
use s3::s3_config;
use dotenv::dotenv;
use std::env;

mod common;
mod errors;
mod index;
mod s3;

#[derive(Debug, Clone)]
struct Secrets {
    key: String,
    aws_access_key: String,
    aws_secret_access_key: String,
}

impl Secrets {
    fn new() -> Self {
        dotenv().ok();
        Secrets { 
            key: env::var("ENC_KEY").expect("ENC_KEY must be set"),
            aws_access_key: env::var("AWS_ACCESS_KEY").expect("AWS_ACCESS_KEY must be set"),
            aws_secret_access_key: env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY must be set"),
        }
    }

    fn aws_creds(&self) -> Credentials {
        Credentials::from_keys(
            self.aws_access_key.clone(), 
            self.aws_secret_access_key.clone(), 
            None,
        )
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let secrets = Secrets::new();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(s3_config)
            .configure(index_config)
    })
    .bind(("localhost", 8123))?
    .run()
    .await
}
