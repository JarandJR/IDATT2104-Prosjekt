use serde::{Deserialize, Serialize};
use std::io::{self};
use std::net::{SocketAddr, UdpSocket};

const STANDARD_PORT: u32 = 8080;
const COMMUNICATION_RADIUS: f32 = 100.0;

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

#[derive(Debug, Serialize, Deserialize)]
struct RoutingTable {
    neighbors: Vec<Neighbor>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Neighbor {
    id: usize,
    position: Coordinate,
}

pub struct Drone {
    id: usize,
    position: Coordinate,
    speed: f32,
    socket: UdpSocket,
    simulator_address: SocketAddr,
    routing_table: RoutingTable,
    go_home: bool,
}

impl Drone {
    pub fn new(
        id: usize,
        position: Coordinate,
        speed: f32,
        simulator_address: SocketAddr,
        go_home: bool,
    ) -> io::Result<Drone> {
        let port = STANDARD_PORT + id as u32;
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port))?;
        Ok(Drone {
            id,
            position,
            speed,
            socket,
            simulator_address,
            routing_table: RoutingTable {
                neighbors: Vec::new(),
            },
            go_home,
        })
    }

    fn move_towards(&mut self, target: &Coordinate) {
        let dx = target.x - self.position.x;
        let dy = target.y - self.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        let steps = (distance / self.speed).ceil() as u32;
        let mut step_x = dx / steps as f32;
        let mut step_y = dy / steps as f32;

        if step_x.is_nan() && step_y.is_nan() {
            self.go_home = true;
        }
        if step_x.is_nan() {
            step_x = 0.0;
        }
        if step_y.is_nan() {
            step_y = 0.0;
        }

        let furthes_neighbor = self.find_furthest_neighbor();
        if let Some(neighbor) = furthes_neighbor {
            println!(
                "Distance between drones: {}",
                self.calculate_distance(
                    step_x + self.position.x,
                    step_y + self.position.y,
                    neighbor.position.clone()
                )
            );
            if self.calculate_distance(
                step_x + self.position.x,
                step_y + self.position.y,
                neighbor.position.clone(),
            ) > COMMUNICATION_RADIUS
            {
                self.send_move_request(&neighbor, target);
            } else {
                println!("Moving towards target: {:?}", target);
                self.move_self(step_x, step_y);
            }
        } else {
            self.move_self(step_x, step_y);
            println!("No neighbors found to relay move request. Moving self");
        }
    }

    fn move_self(&mut self, step_x: f32, step_y: f32) {
        self.position.x += step_x;
        self.position.y += step_y;

        // Update position
        self.send_position_to_simulator();
        self.update_neighbors();
        println!("Current position: {:?}", self.position);
    }

    fn calculate_distance(&self, x: f32, y: f32, target: Coordinate) -> f32 {
        let dx = target.x - x;
        let dy = target.y - y;
        (dx * dx + dy * dy).sqrt()
    }

    fn update_neighbors(&self) {
        for neighbor in &self.routing_table.neighbors {
            let message = format!("UPDATE {} {} {}", self.id, self.position.x, self.position.y);
            let port = STANDARD_PORT + neighbor.id as u32;
            let neighbor_address: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
            
            self.socket
                .send_to(message.as_bytes(), neighbor_address)
                .unwrap();
        }
    }

    fn update_neighbor(&mut self, message: &str) {
        let message_parts: Vec<&str> = message.trim().split_whitespace().collect();

        println!("updating neighbor");
        if message_parts.len() == 4 {
            if let (Ok(requester_id), Ok(requester_x), Ok(requester_y)) = (
                message_parts[1].parse::<usize>(),
                message_parts[2].parse::<f32>(),
                message_parts[3].parse::<f32>(),
            ) {
                for neighbor in &mut self.routing_table.neighbors {
                    if neighbor.id == requester_id {
                        neighbor.position.x = requester_x;
                        neighbor.position.y = requester_y;
                        println!("neighbor updated {}", neighbor.id);
                    }
                }
            }
        }
    }

    fn send_position_to_simulator(&self) {
        let drone_data = DroneData {
            id: self.id,
            x: self.position.x,
            y: self.position.y,
        };
        let message = serde_json::to_string(&drone_data).unwrap();

        self.socket
            .send_to(message.as_bytes(), self.simulator_address)
            .unwrap();

        println!("Sent position update to simulator: {:?}", self.position);
    }

    fn send_finished_to_simulator(&self) {
        let message = "goHome: true";
        self.socket
            .send_to(message.as_bytes(), self.simulator_address)
            .unwrap();

        println!("Sent finished update to simulator: {:?}", self.go_home);
    }

    fn send_move_request(&self, neighbor: &Neighbor, target: &Coordinate) {
        let move_request = format!(
            "MOVE_REQUEST {} {} {}",
            self.id, target.x, target.y
        );
        let port = STANDARD_PORT + neighbor.id as u32;
        let neighbor_address: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

        self.socket
            .send_to(move_request.as_bytes(), neighbor_address)
            .unwrap();

        println!(
            "Sent move request to neighbor {}: {:?}",
            neighbor.id, target
        );
    }

    fn find_furthest_neighbor(&self) -> Option<&Neighbor> {
        self.routing_table.neighbors.iter().max_by(|n1, n2| {
            let distance1 = ((n1.position.x - self.position.x).powi(2)
                + (n1.position.y - self.position.y).powi(2))
            .sqrt();
            let distance2 = ((n2.position.x - self.position.x).powi(2)
                + (n2.position.y - self.position.y).powi(2))
            .sqrt();
            distance1.partial_cmp(&distance2).unwrap()
        })
    }

    fn receive_move_request(&mut self, message: &str) {
        let message_parts: Vec<&str> = message.trim().split_whitespace().collect();

        if message_parts.len() == 4 {
            if let (
                Ok(requester_id),
                Ok(target_x),
                Ok(target_y),
            ) = (
                message_parts[1].parse::<usize>(),
                message_parts[2].parse::<f32>(),
                message_parts[3].parse::<f32>(),
            ) {
                let mut request_is_neighbor = false;
                for neighbor in &self.routing_table.neighbors {
                    if neighbor.id == requester_id {
                        request_is_neighbor = true;
                    }
                }

                if !request_is_neighbor {
                    println!("Requester is not a neighbor: {}", requester_id);
                    return;
                }

                let target = Coordinate {
                    x: target_x,
                    y: target_y,
                };

                self.move_towards(&target);
            }
        } else {
            println!("Invalid move request format.");
        }
    }

    fn is_within_communication_radius(&self, x: f32, y: f32) -> bool {
        let distance = ((x - self.position.x).powi(2) + (y - self.position.y).powi(2)).sqrt();
        distance <= COMMUNICATION_RADIUS
    }

    fn receive_target_from_simulator(&self, message: &str) -> Option<Coordinate> {
        if message == "stop" {
            println!("Stopped loop");
            return None;
        }

        let message_parts: Vec<&str> = message.trim().split_whitespace().collect();
        if let (Ok(neighbor_x), Ok(neighbor_y)) = (
            message_parts[1].parse::<f32>(),
            message_parts[2].parse::<f32>(),
        ) {
            let target = Coordinate {
                x: neighbor_x,
                y: neighbor_y,
            };
            println!("Received target from simulator: {:?}", target);
            Option::from(target)
        } else {
            println!("Error reading coordinate");
            None
        }
    }

    fn receive_and_send_message(&mut self, message: &str) {
        let message_parts: Vec<&str> = message.trim().split_whitespace().collect();
        if let (Ok(sender), Ok(message_sent)) = (
            message_parts[1].parse::<usize>(),
            message_parts[2].parse::<String>(),
        ) {
            println!("Message received: {}", message_sent);
            self.go_home = true;
            self.send_messages(sender, message_sent);
        }
    }

    fn send_messages(&self, sender: usize, message: String) {
        for neighbor in &self.routing_table.neighbors {
            if neighbor.id != sender {
                self.send_message(&message, neighbor.id, "MESSAGE");
            }
        }
    }

    fn send_message(&self, message: &str, to: usize, message_type: &str) {
        let message = format!("{} {} {}", message_type, self.id, message);
        let port = STANDARD_PORT + to as u32;
        let neighbor_address: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();

        self.socket
            .send_to(message.as_bytes(), neighbor_address)
            .unwrap();

        println!("Sent message to neighbor {}: {}", to, message);
    }

    fn add_neighbor(&mut self, message: &str) {
        let message_parts: Vec<&str> = message.trim().split_whitespace().collect();
        if let (Ok(id), Ok(neighbor_x), Ok(neighbor_y)) = (
            message_parts[1].parse::<usize>(),
            message_parts[2].parse::<f32>(),
            message_parts[3].parse::<f32>(),
        ) {
            let position = Coordinate {
                x: neighbor_x,
                y: neighbor_y,
            };

            if self.is_within_communication_radius(neighbor_x, neighbor_y) {
                let mut contains = false;
                for neighbor in &self.routing_table.neighbors {
                    if neighbor.id == id {
                        contains = true;
                    }
                }

                if !contains {
                    println!("Neighbor {} added", id);
                    self.routing_table.neighbors.push(Neighbor { id, position });
                    let neighbor_message = format!("{} {}", neighbor_x, neighbor_y);
                    self.send_message(&neighbor_message, id, "ADD_NEIGHBOR")
                }
            } else {
                println!("Neighbor to far away.");
            }
        }
    }

    pub fn run(&mut self) {
        let mut buffer = [0u8; 1024];

        while !self.go_home {
            println!("Waiting for request...");
            if let Ok((size, _)) = self.socket.recv_from(&mut buffer) {
                let message = std::str::from_utf8(&buffer[..size]).unwrap();
                let message_parts: Vec<&str> = message.trim().split_whitespace().collect();
                if message_parts[0] == "MOVE_REQUEST" {
                    self.receive_move_request(message);
                } else if message_parts[0] == "ADD_NEIGHBOR" {
                    self.add_neighbor(message)
                } else if message_parts[0] == "MESSAGE" {
                    self.receive_and_send_message(message)
                } else if message_parts[0] == "UPDATE" {
                    self.update_neighbor(message);
                } else if message_parts[0] == "POSITION"{
                    self.send_position_to_simulator();
                } else if message_parts[0] == "MOVE" {
                    let target = self.receive_target_from_simulator(message);
                    if let Some(target) = target {
                        self.move_towards(&target);
                    } else {
                        println!("Exit program");
                        break;
                    }
                }
            }
        }

        if self.go_home {
            self.send_finished_to_simulator();
            self.send_messages(self.id, String::from("TEST"));
        }
    }
}
