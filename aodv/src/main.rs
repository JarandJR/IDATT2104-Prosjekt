mod simulator;

use std::{fs::File, io::{Read, BufReader, BufRead, self}, env};

use simulator::Simulator;

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sim = Simulator::new(3);

    let drones_file_path = "droner.txt";
    let lines = read_file(drones_file_path).unwrap();
    let drones = make_drones(lines);

    let connection_file_path = "kobling_droner.txt";
    let lines = read_file(connection_file_path).unwrap();
    make_graph(lines, drones);


    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(test_connection)
            .service(do_step)
            .service(get_drones)
            .service(is_finished)
            .data(sim.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/test_connection")]
async fn test_connection(sim: web::Data<Simulator>)  -> impl Responder {
    HttpResponse::Ok().json(format!("Connected, {}", sim.drones))
}

#[get("/do_step")]
async fn do_step(sim: web::Data<Simulator>) -> impl Responder {
    println!("doing step");
    HttpResponse::Ok().json(format!("Test of appstate: 3 = , {}", sim.drones))
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

#[derive(Debug,Clone)]
pub struct Graph {
    pub drones: Vec<Vec<Drone>>,
}

impl Graph {
    fn with_nodes(amount_of_nodes: usize) -> Self {
        let mut drones: Vec<Vec<Drone>> = Vec::with_capacity(amount_of_nodes);
        for _ in 0..amount_of_nodes {
            drones.push(Vec::new());
        }
        Self { drones }
    }
}

#[derive(Debug, Clone)]
pub struct Drone {
    pub id: usize,
    pub x_coordinates: usize,
    pub y_coordinates: usize,
    pub speed: usize,
}


impl Drone {
    pub fn new(id: usize, x_coordinates: usize, y_coordinates: usize, speed: usize) -> Drone {
        Drone {
            id,
            x_coordinates,
            y_coordinates,
            speed,
        }
    }
}


fn read_file(path: &str) -> Option<Vec<String>> {

     // Open the file
     let file = File::open(path);

     // Handle any potential errors when opening the file
     let file = match file {
         Ok(f) => f,
         Err(e) => {
             // Print the error message and return early
             println!("Error opening the file: {}", e);
             return None;
         }
     };
 
     let reader = BufReader::new(file);

     let lines: Vec<String> = reader
         .lines()
         .into_iter()
         .map(|line| line.expect("Failed to read line"))
         .collect();

    Some(lines)
}

fn make_drones(lines: Vec<String>) -> Vec<Drone> {

    let mut drones: Vec<Drone> = Vec::new();

    let num_of_drones = lines.len() - 1;
    for i in 1..= num_of_drones {

        let data: Vec<usize> = lines[i]
            .split_whitespace()
            .map(|c| c.parse().expect("File not formated correctly"))
            .collect();

        let id = data[0];
        let x_coordinates = data[1];
        let y_coordinates = data[2]; 
        let speed = 1;

        let drone = Drone::new(
           id,
           x_coordinates,
           y_coordinates,
           speed
        );
        
         drones.push(drone);
    }

    drones
}

fn make_graph(lines: Vec<String>, drones: Vec<Drone>) -> io::Result<Graph> {

    let first_line: Vec<&str> = lines[0].split_whitespace().collect();
    let nodes: usize = first_line[0].parse().expect("File not formated correctly");
    let edges: usize = first_line[1].parse().expect("File not formated correctly");
    println!("{} - {}", lines.len(), edges + 1);
    assert!(lines.len() == edges + 1);

    let mut graph = Graph::with_nodes(nodes);
    for i in 1..=edges {
        
        let data: Vec<usize> = lines[i]
            .split_whitespace()
            .map(|c| c.parse().expect("File not formated correctly"))
            .collect();

        let drone = drones[data[1]].clone();
        graph.drones[data[0]].push(drone);
    }

    println!("{:?}", graph.drones);

    Ok(graph)
}
