mod simulator;

use simulator::Coordinate;
use simulator::Simulator;

use actix_cors::Cors;
use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sim = Simulator::new();
    println!("started");

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
async fn test_connection() -> impl Responder {
    HttpResponse::Ok().json("Connected")
}

#[post("/do_step")]
async fn do_step(sim: web::Data<Simulator>, coor: web::Json<Coordinate>) -> impl Responder {
    println!("doing step");
    let x = coor.x;
    let y = coor.y;

    sim.do_step(x, y);
    HttpResponse::Ok()
}

#[get("/get_drones")]
async fn get_drones(sim: web::Data<Simulator>) -> impl Responder {
    println!("getting drones");
    HttpResponse::Ok().json(sim.get_drones())
}

#[get("/is_finished")]
async fn is_finished(sim: web::Data<Simulator>) -> impl Responder {
    println!("Checks if simulations is finished");
    HttpResponse::Ok().json(sim.is_finished())
}

#[put("/update")]
async fn update_drones(sim: web::Data<Simulator>) -> impl Responder {
    sim.update_drones();
    HttpResponse::Ok()
}
