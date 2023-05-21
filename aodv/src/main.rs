mod simulator;

use simulator::Simulator;
use simulator::run_drones;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sim = Simulator::new();
    run_drones(&sim);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(test_connection)
            .service(do_step)
            .service(get_drones)
            .service(is_finished)
            .data(sim.clone())
    })
    .bind(("127.0.0.1", 8079))?
    .run()
    .await
}

#[get("/test_connection")]
async fn test_connection(sim: web::Data<Simulator>)  -> impl Responder {
    HttpResponse::Ok().json(format!("Connected, {:?}", sim.drones))
}

#[get("/do_step")]
async fn do_step(sim: web::Data<Simulator>) -> impl Responder {
    println!("doing step");
    HttpResponse::Ok().json(format!("Test of appstate: 3 = , {:?}", sim.drones))
}

#[get("/get_drones")]
async fn get_drones(sim: web::Data<Simulator>) -> impl Responder {
    println!("getting drones");
    let mut drones: Vec<Vec<usize>> = Vec::new();
    let mut drone: Vec<usize> = Vec::new();
    drone.push(1);
    drone.push(20);
    drone.push(50);
    drones.push(drone);

    let mut drone2: Vec<usize> = Vec::new();
    drone2.push(2);
    drone2.push(10);
    drone2.push(500);
    drones.push(drone2);
    HttpResponse::Ok().json(drones)
}

#[get("/is_finished")]
async fn is_finished(sim: web::Data<Simulator>) -> impl Responder {
    println!("IS FINISHED?");
    HttpResponse::Ok().json(String::from("Test is finished"))
}






