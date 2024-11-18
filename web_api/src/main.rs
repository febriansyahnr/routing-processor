mod transfer;

use actix_web::{App, HttpServer};
use transfer::*;
use core::prelude::*;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("Starting web api");

    HttpServer::new(move || {
        App::new()
            .service(handle_transfer)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
