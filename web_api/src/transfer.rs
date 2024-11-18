use core::model::transfer::TransferRequest;
use actix_web::{post, web, HttpResponse, Responder};


#[post("/api/internal/v1.0/transfer")]
pub async fn handle_transfer(transfer: web::Json<TransferRequest>) -> impl Responder {
    HttpResponse::Ok().json(transfer)
}