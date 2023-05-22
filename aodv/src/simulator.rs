use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    net::UdpSocket,
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DroneData {
    id: usize,
    x: f32,
    y: f32,
}

#[derive(Debug, Clone)]
struct Graph {
    drones: Vec<Vec<Drone>>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drone {
    id: usize,
    x: f32,
    y: f32,
}

impl Drone {
    fn new(id: usize, x: f32, y: f32) -> Drone {
        Drone { id, x, y }
    }
}

#[derive(Clone)]
pub struct Simulator {
    graph: Graph,
    pub drones: Arc<Mutex<Vec<Drone>>>,
    finished: Arc<Mutex<bool>>,
}

impl Simulator {
    pub fn new() -> Self {
        let drones_file_path = "droner.txt";
        let lines = read_file(drones_file_path).unwrap();
        let drones = make_drones(lines);

        let connection_file_path = "kobling_droner.txt";
        let lines = read_file(connection_file_path).unwrap();
        let graph = make_graph(lines, drones.lock().unwrap().clone());
        let sim = Self {
            graph,
            drones,
            finished: Arc::new(Mutex::new(false)),
        };
        sim.start_reader_thread();
        make_edges(&sim);
        run_drones(&sim);

        sim
    }

    pub fn do_step(&self, x: f32, y: f32) {
        let standard_port = 8080;
        let url = "127.0.0.1:";
        let message = "MOVE";

        let socket = UdpSocket::bind("127.0.0.1:7879").expect("Could not bind socket");
        for drone in &*self.drones.lock().unwrap() {
            println!("Drone {}: x: {}, y: {}", drone.id, drone.x, drone.y);
            let port = standard_port + drone.id;
            socket
                .send_to(
                    format!("{} {} {}", message, x, y).as_bytes(),
                    format!("{}{}", url, port),
                )
                .unwrap();
        }
        println!("Sent all the messages to the drones");
    }

    fn start_reader_thread(&self) {
        let drones_clone = self.drones.clone();
        let finished_clone = self.finished.clone();

        thread::spawn(move || {
            let socket = UdpSocket::bind("0.0.0.0:7878").expect("Could not bind socket");
            loop {
                let mut buffer = [0u8; 1024];
                if let Ok((size, _)) = socket.recv_from(&mut buffer) {
                    let message = std::str::from_utf8(&buffer[..size]).unwrap();
                    if let Ok(parsed) = message.parse::<bool>() {
                        if parsed {
                            println!("Target reached");
                            *finished_clone.lock().unwrap() = true;
                            break;
                        }
                    }
                    let data = Some(serde_json::from_str::<DroneData>(message).unwrap());

                    if let Some(data) = data {
                        let dron_data: DroneData = data;
                        let mut drones = drones_clone.lock().unwrap();
                        for drone in &mut *drones {
                            if drone.id == dron_data.id {
                                drone.x = dron_data.x;
                                drone.y = dron_data.y;
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn get_drones(&self) -> Vec<Drone> {
        self.drones.lock().unwrap().clone()
    }

    pub fn update_drones(&self) {
        let standard_port = 8080;
        let socket = UdpSocket::bind("127.0.0.1:7879").expect("Could not bind socket");

        for drone in &*self.drones.lock().unwrap() {
            let port = standard_port + drone.id;
            socket
                .send_to("POSITION".as_bytes(), format!("127.0.0.1:{}", port))
                .unwrap();
        }
    }

    pub fn is_finished(&self) -> bool {
        *self.finished.lock().unwrap()
    }
}

fn read_file(path: &str) -> Option<Vec<String>> {
    let file = File::open(path);

    // Handle any potential errors when opening the file
    let file = match file {
        Ok(f) => f,
        Err(e) => {
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

fn make_drones(lines: Vec<String>) -> Arc<Mutex<Vec<Drone>>> {
    let drones: Arc<Mutex<Vec<Drone>>> = Arc::new(Mutex::new(Vec::new()));

    let num_of_drones = lines.len() - 1;
    for i in 1..=num_of_drones {
        let data: Vec<usize> = lines[i]
            .split_whitespace()
            .map(|c| c.parse().expect("File not formated correctly"))
            .collect();

        let id = data[0];
        let x_coordinates = data[1] as f32;
        let y_coordinates = data[2] as f32;

        let drone = Drone::new(id, x_coordinates, y_coordinates);

        drones.lock().unwrap().push(drone);
    }

    drones
}

fn make_graph(lines: Vec<String>, drones: Vec<Drone>) -> Graph {
    let first_line: Vec<&str> = lines[0].split_whitespace().collect();
    let nodes: usize = first_line[0].parse().expect("File not formated correctly");
    let edges: usize = first_line[1].parse().expect("File not formated correctly");

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
    graph
}

fn run_drones(sim: &Simulator) {
    for drone in &*sim.drones.lock().unwrap() {
        run_drone_windows(drone.id, drone.x, drone.y);
    }
    println!("Waiting for drones to start");
    thread::sleep(std::time::Duration::from_secs(10));
    println!("Adding edges");
    make_edges(sim);
}

fn get_path_to_drone() -> String {
    let current_dir = env::current_dir().expect("could not find the correct file path");
    let project_path = PathBuf::from(current_dir)
        .join("../drone")
        .canonicalize()
        .expect("could not modefy the file path");
    let path: String = format!("{}", project_path.display());
    path
}

fn run_drone_windows(id: usize, x: f32, y: f32) -> std::io::Result<()> {
    let project_path = get_path_to_drone();
    let trimmed_path = project_path.trim_start_matches(&['\\', '?'][..]);

    // Command to run your application
    let command = format!("cargo run {} {} {}", id, x, y);

    // Run the command in a new cmd window
    let output = Command::new("cmd.exe")
        .arg("/C")
        .arg("start")
        .arg("cmd.exe")
        .arg("/K")
        .arg(&command)
        .current_dir(trimmed_path) // Set the working directory to your project's directory
        .spawn()?;

    Ok(())
}

fn make_edges(sim: &Simulator) {
    let standard_port = 8080;
    let url = "127.0.0.1:";
    let socket = UdpSocket::bind("127.0.0.1:7877").expect("Could not bind socket");
    let message = "ADD_NEIGHBOR";
    for (drone, _) in (0..sim.graph.drones.len()).enumerate() {
        for (edge, _) in (0..sim.graph.drones[drone].len()).enumerate() {
            let port = standard_port + drone;
            let id = sim.graph.drones[drone][edge].id;
            let x = sim.graph.drones[drone][edge].x;
            let y = sim.graph.drones[drone][edge].y;
            socket
                .send_to(
                    format!("{} {} {} {}", message, id, x, y).as_bytes(),
                    format!("{}{}", url, port),
                )
                .unwrap();
        }
    }
}
