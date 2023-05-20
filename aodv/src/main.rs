mod simulator;

use simulator::Simulator;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sim = Simulator::new(3);
    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(test_connection)
            .service(do_step)
            .service(get_drones_position)
            .service(is_finished)
            .data(sim.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/test_connection")]
async fn test_connection(sim: web::Data<Simulator>)  -> String {
    format!("Connected, {}", sim.drones)
}

#[get("/do_step")]
async fn do_step(sim: web::Data<Simulator>) -> String {
    format!("Test of appstate: 3 = , {}", sim.drones)
}

#[get("/get_drones_position")]
async fn get_drones_position(sim: web::Data<Simulator>) -> String {
    String::from("Test")
}

#[get("/is_finished")]
async fn is_finished(sim: web::Data<Simulator>) -> String {
    String::from("Test")
}
