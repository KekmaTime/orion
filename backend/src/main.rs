use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::app_state::AppState;
use crate::payload::PaymentSessionResponse;

mod app_state;
mod payload;
mod payment;

async fn create_payment_session(state: web::Data<AppState>) -> impl Responder {
    let url = state.payment.create_stripe_session().await;
    let response  = PaymentSessionResponse{
        url
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let app_state = web::Data::new(AppState::new());


    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .route("/checkout", web::post().to(create_payment_session))
    })

        .bind("0.0.0.0:4000")
        .unwrap()
        .run()
        .await
}