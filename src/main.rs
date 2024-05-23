use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use index::index_config;
use s3::s3_config;

mod common;
mod errors;
mod index;
mod s3;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

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
