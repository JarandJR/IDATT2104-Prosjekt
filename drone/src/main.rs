mod drone;

use std::env;
use std::io;
use std::net::SocketAddr;

use drone::{Coordinate, Drone};

fn main() -> io::Result<()> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the required number of arguments is provided
    if args.len() != 4 {
        eprintln!("Usage: program_name id x y");
        std::process::exit(1);
    }

    // Parse command-line arguments
    let id: usize = args[1].parse().expect("Could not parse drone ID");
    let x: f32 = args[2]
        .parse()
        .expect("Could not parse initial position (x)");
    let y: f32 = args[3]
        .parse()
        .expect("Could not parse initial position (y)");

    // Address of the simulator program
    let simulator_address: SocketAddr = "127.0.0.1:7878".parse().unwrap();

    let position = Coordinate { x, y };

    let mut drone = Drone::new(id, position, simulator_address, false)?;

    drone.run();

    Ok(())
}
