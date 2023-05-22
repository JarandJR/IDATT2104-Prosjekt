use std::{fs::File, io::{BufReader, BufRead, self}, process::Command, net::UdpSocket, env, path::{PathBuf, Display, Path}, fmt::format};
use dirs;

#[derive(Debug,Clone)]
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

#[derive(Debug, Clone)]
pub struct Drone {
    id: usize,
    x_coordinates: usize,
    y_coordinates: usize,
}

impl Drone {
    fn new(id: usize, x_coordinates: usize, y_coordinates: usize) -> Drone {
        Drone {
            id,
            x_coordinates,
            y_coordinates,
        }
    }
}

#[derive(Clone)]
pub struct Simulator {
    graph: Graph,
    pub drones: Vec<Drone>
}

impl Simulator {
    pub fn new() -> Self {
        let drones_file_path = "droner.txt";
        let lines = read_file(drones_file_path).unwrap();
        let drones = make_drones(lines);

        let connection_file_path = "kobling_droner.txt";
        let lines = read_file(connection_file_path).unwrap();
        let graph = make_graph(lines, drones.clone());
        Self {
            graph,
            drones
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

        let drone = Drone::new(
           id,
           x_coordinates,
           y_coordinates
        );
        
         drones.push(drone);
    }

    drones
}

fn make_graph(lines: Vec<String>, drones: Vec<Drone>) -> Graph {

    let first_line: Vec<&str> = lines[0].split_whitespace().collect();
    let nodes: usize = first_line[0].parse().expect("File not formated correctly");
    let edges: usize = first_line[1].parse().expect("File not formated correctly");
    //println!("{} - {}", lines.len(), edges + 1);
    //println!("{:?}", lines);

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

pub fn run_drones(sim: &Simulator) {
    
    let output = if cfg!(target_os = "windows") {
        //for drone in &sim.drones {
            //println!("{:?}",drone);
            run_drone_in_docker_windows(1, 20, 20);
        //}
    } else {
        run_drone_in_docker_unix(1, 1, 1);
    };
    // make_edges(sim);
}

fn get_path_to_drone() -> String {
    let current_dir = env::current_dir().expect("could not find the correct file path");
    let project_path = PathBuf::from(current_dir).join("../drone")
                                .canonicalize()
                                .expect("could not modefy the file path");
    let path: String = format!("{}", project_path.display());
    path
}

fn run_drone_in_docker_windows(id:usize, x: usize, y:usize) -> std::io::Result<()> {
    // Windows path to your Rust project
    // Make sure to replace it with your actual project path
    
    let project_path = get_path_to_drone();
    let trimmed_path = project_path.trim_start_matches(&['\\', '?'][..]);

    let cargo_registry_path = get_cargo_registry_path().unwrap();
    let port = 8080 + id;

     // Docker command to run `cargo run` in the container
     let docker_command = format!(
        "docker run -v {}:/usr/src/myapp -v {}:/usr/local/cargo/registry -p {}:{}/udp -w /usr/src/myapp -it rust:latest cargo run -- {} {} {}",
        trimmed_path,
        cargo_registry_path,
        port,
        port,
        id,
        x,
        y
    );

    // Run the Docker command in a new cmd window
    let output = Command::new("cmd.exe")
        .arg("/C")
        .arg("start")
        .arg("cmd.exe")
        .arg("/C")
        .arg(&docker_command)
        .spawn()?;

    Ok(())
}

fn get_cargo_registry_path() -> Option<String> {
    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".cargo/registry");
            Some(path.to_str()?.to_string())
        },
        None => None,
    }
}

fn run_drone_in_docker_unix(id:usize, x: usize, y:usize) -> io::Result<()> {
    // Path to your Rust project
   
    let project_path = get_path_to_drone();

    // Docker command to run `cargo run` in the container
    let docker_command = format!(
        "docker run -v {}:/usr/src/myapp -w /usr/src/myapp -it rust:latest cargo run -- {} {} {}",
        project_path,
        id,
        x,
        y
    );

    // Escape the double quotes in the Docker command
    let escaped_docker_command = docker_command.replace("\"", "\\\"");

    // Use powershell to open a new terminal window and run the Docker command
    let output = Command::new("powershell.exe")
        .arg("start")
        .arg("wsl")
        .arg("\"")
        .arg(&escaped_docker_command)
        .arg("\"")
        .spawn()?;
    Ok(())
}

fn make_edges(sim: &Simulator) {
    let standard_port = 8080;
    let url = "127.0.0.1:";
    let socket = UdpSocket::bind("127.0.0.1:7878").expect("Could not bind socket");
    let message = "ADD_NEIGHBOR";
    for (drone, _) in (0..sim.graph.drones.len()).enumerate() {
        for (edge, _) in (0..sim.graph.drones[drone].len()).enumerate() {
            let port = standard_port +  drone;
            let id = sim.graph.drones[drone][edge].id;
            let x = sim.graph.drones[drone][edge].x_coordinates;
            let y = sim.graph.drones[drone][edge].y_coordinates;
            socket.send_to(format!("{} {} {} {}", message, id, x, y).as_bytes(), format!("{}{}", url, port))
            .unwrap();
        }   
    }
}

