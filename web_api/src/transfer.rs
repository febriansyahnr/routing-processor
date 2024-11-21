use core::{model::transfer::TransferRequest, port::service::CanTransfer};
use std::collections::HashMap;
use actix_web::{post, web, HttpResponse, Responder};
use core::prelude::*;

#[post("/api/v1.0/internal/transfer")]
pub async fn handle_transfer(
    state: web::Data<AppState>, 
    req: web::Json<TransferRequest>,
) -> impl Responder {
    let transfer_service = state.transfer_serice.as_ref();
    let response = transfer_service.transfer(&req).await;
    match response {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(e) => {
            let mut response = HashMap::new();
            response.insert("error", e.to_string());
            HttpResponse::InternalServerError().json(response)
        }
    }
}