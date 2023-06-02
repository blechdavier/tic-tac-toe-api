use actix_web::{middleware, App, HttpServer};
use std::{env, io};

mod tictactoe;

#[actix::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    // env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(tictactoe::get_best_move)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
