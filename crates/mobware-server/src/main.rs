use actix::{Actor, Arbiter};
use actix_web::{get, App, HttpServer, Responder};
use log;
use mobware_core::{Developer, Orchestrator};

#[get("/alive")]
async fn alive() -> impl Responder {
    format!("Alive")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log::info!("Starting server");

    Arbiter::new().spawn(async {
        Developer::new("Bob".to_string()).start();
    });

    Arbiter::new().spawn(async {
        Orchestrator::new("Alice".to_string()).start();
    });

    let result = HttpServer::new(|| App::new().service(alive))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await;
    log::info!("Stopping server");
    result
}
