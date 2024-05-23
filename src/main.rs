use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use aws_credential_types::Credentials;
use aws_sdk_s3::config::{timeout::TimeoutConfig, Builder as S3Builder, Region};
use env_logger::Env;
use index::index_config;
use s3::s3_config;

mod common;
mod errors;
mod index;
mod s3;

const SECRETS_JSON: &str = include_str!("../secrets.json");

#[derive(Debug, Clone, serde::Deserialize)]
struct Secrets {
    #[serde(rename = "ENC_KEY")]
    key: String,
    #[serde(rename = "AWS_ACCESS_KEY")]
    aws_access_key: String,
    #[serde(rename = "AWS_SECRET_ACCESS_KEY")]
    aws_secret_access_key: String,
}

impl Secrets {
    fn aws_creds(&self) -> Credentials {
        Credentials::from_keys(
            self.aws_access_key.clone(),
            self.aws_secret_access_key.clone(),
            None,
        )
    }
}

async fn create_s3_client(provider: &Secrets) -> aws_sdk_s3::Client {
    let config = aws_config::from_env()
        .region(Region::new("af-south-1"))
        .credentials_provider(provider.aws_creds())
        .load()
        .await;

    let timeout_config = TimeoutConfig::builder().build();

    let s3_config = S3Builder::from(&config)
        .timeout_config(timeout_config)
        .build();

    aws_sdk_s3::Client::from_conf(s3_config)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let secrets: Secrets =
        serde_json::from_str(SECRETS_JSON).expect("Failed to parse the secrets json");

    let s3_client = create_s3_client(&secrets).await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(s3_client.clone()))
            .wrap(Logger::default())
            .configure(s3_config)
            .configure(index_config)
    })
    .bind(("localhost", 8123))?
    .run()
    .await
}
