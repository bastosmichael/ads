#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

mod constants;
mod click;
mod response;
mod ad;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(ad::list)
            .service(ad::get)
            .service(ad::create)
            .service(ad::delete)
            .service(click::list)
            .service(click::redirect)
            .service(click::reject)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
