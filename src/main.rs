use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main()-> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
    })
    .bind(("localhost", 8123))?
    .run()
    .await
}
