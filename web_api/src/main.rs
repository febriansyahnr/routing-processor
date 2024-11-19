mod transfer;

use core::port::service::EProcessors;
use std::collections::HashMap;
use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

use transfer::*;
use core::prelude::*;
use core::config::get_config;
use core::utils::*;
use core::repository::processors;
use core::repository as repositories;
use core::service::transfer as services;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = get_config();
    let mysql_pool = connection::get_mysql_pool(&config.database_url).await?;

    // inistialize repositories
    let snap_core_repo = Box::new(
        EProcessors::SnapCore(
            processors::snap_core_processor::SnapCoreProcessor::new()
        )
    );
    let xb_core_repo = Box::new(
        EProcessors::XBCore(
            processors::xb_core_processor::XBCoreProcessor::new()
        )
    );
    let mut core_processors: HashMap<String, Box<EProcessors>> = HashMap::new();
    core_processors.insert("snap-core-processor".to_string(), snap_core_repo);
    core_processors.insert("xb-core-processor".to_string(), xb_core_repo);
    
    let transfer_config_repo = Box::new(
        repositories::transfer_config::TransferConfigRepository::new(mysql_pool)
    );

    //inistialize services
    let transfer_service = services::TransferService::new(transfer_config_repo, core_processors);

    // setup app state
    let app_state = AppState {
        transfer_serice: Arc::new(transfer_service),
    };

    println!("Starting web api");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(app_state.clone()))
            .service(handle_transfer)
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
