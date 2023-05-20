mod simulator;

use simulator::Simulator;

use actix_web::{get, post, web::Json, App, HttpServer};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(test_connection)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/test_connection")]
async fn test_connection()  -> String {
    let sim = Simulator::new(3);
    format!("Connected, {}", sim.drones)
}
