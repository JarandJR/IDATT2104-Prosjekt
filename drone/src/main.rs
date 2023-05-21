mod drone;

use std::error::Error;
use std::io::{self};
use std::net::SocketAddr;
use std::str::FromStr;

use drone::{Coordinate, Drone};

fn main() -> io::Result<()> {
    // User input for drone configuration
    println!("Enter drone ID:");
    let id: usize = read_input()?;
    println!("Enter initial position (x):");
    let x: f32 = read_input()?;
    println!("Enter initial position (y):");
    let y: f32 = read_input()?;
    let position: Coordinate = Coordinate { x, y };
    println!("Enter speed:");
    let speed: f32 = read_input()?;

    // Address of the simulator program
    let simulator_address: SocketAddr = "127.0.0.1:7878".parse().unwrap();

    let mut drone = Drone::new(id, position, speed, simulator_address, false)?;

    drone.run();

    Ok(())
}

fn read_input<T>() -> io::Result<T>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    input
        .trim()
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
