
use serde::{Deserialize, Serialize};
use std::io::{self};
use std::net::{SocketAddr, UdpSocket};

const STANDARD_PORT:u32 = 8080;

#[derive(Debug, Serialize, Deserialize)]
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
    position:Coordinate,
    speed: f32,
    communication_radius: f32,
    socket: UdpSocket,
    simulator_address: SocketAddr,
    routing_table: RoutingTable,
    go_home: bool
}

impl Drone {
    pub fn new(
        id: usize,
        position: Coordinate,
        speed: f32,
        communication_radius: f32,
        simulator_address: SocketAddr,
        go_home: bool
    ) -> io::Result<Drone> {
        let port = STANDARD_PORT + id as u32;
        let socket = UdpSocket::bind(format!("127.0.0.1:{}", port))?;
        Ok(Drone {
            id,
            position,
            speed,
            communication_radius,
            socket,
            simulator_address,
            routing_table: RoutingTable {
                neighbors: Vec::new(),
            },
            go_home
        })
    }

    fn move_towards(&mut self, target: &Coordinate) {
        let dx = target.x - self.position.x;
        let dy = target.y - self.position.y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance > self.communication_radius {
            // Target is outside the communication radius, request nearest neighbor to move towards the target
            let nearest_neighbor = self.find_nearest_neighbor();
            if let Some(neighbor) = nearest_neighbor {
                // Send message to neighbor requesting it to move towards the target
                self.send_move_request(&neighbor, target);
            } else {
                println!("No neighbors found to relay move request.");
            }
        } else {
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

            println!("Moving towards target: {:?}", target);
            self.position.x += step_x;
            self.position.y += step_y;

            // Send updated position to simulator program
            self.send_position_to_simulator();

            println!("Current position: {:?}", self.position);
        }
    }

    fn send_position_to_simulator(&self) {
        let drone_data = DroneData {
            id: self.id,
            x: self.position.x,
            y: self.position.y,
        };
        let message = serde_json::to_string(&drone_data).unwrap();
        
        self.socket.send_to(message.as_bytes(), self.simulator_address)
            .unwrap();

        println!("Sent position update to simulator: {:?}", self.position);
    }

    fn send_finished_to_simulator(&self) {
        let message = "goHome: true";
        self.socket.send_to(message.as_bytes(), self.simulator_address)
            .unwrap();

        println!("Sent finished update to simulator: {:?}", self.go_home);
    }

    fn send_move_request(&self, neighbor: &Neighbor, target: &Coordinate) {
        let move_request = format!("MOVE_REQUEST {} {} {} {} {}", self.id, self.position.x, self.position.y, target.x, target.y);
        let neighbor_address: SocketAddr = format!("127.0.0.1:808{}", neighbor.id).parse().unwrap();

        self.socket.send_to(move_request.as_bytes(), neighbor_address)
            .unwrap();

        println!("Sent move request to neighbor {}: {:?}", neighbor.id, target);
    }

    fn find_nearest_neighbor(&self) -> Option<&Neighbor> {
        self.routing_table
            .neighbors
            .iter()
            .min_by(|n1, n2| {
                let distance1 = ((n1.position.x - self.position.x).powi(2) + (n1.position.y - self.position.y).powi(2)).sqrt();
                let distance2 = ((n2.position.x - self.position.x).powi(2) + (n2.position.y - self.position.y).powi(2)).sqrt();
                distance1.partial_cmp(&distance2).unwrap()
            })
    }

    fn receive_move_request(&mut self, message: &str) {
        let message_parts: Vec<&str> = message.trim().split_whitespace().collect();

        if message_parts.len() == 6 {
            if let (Ok(requester_id), Ok(requester_x), Ok(requester_y), Ok(target_x), Ok(target_y)) = (
                message_parts[1].parse::<usize>(),
                message_parts[2].parse::<f32>(),
                message_parts[3].parse::<f32>(),
                message_parts[4].parse::<f32>(),
                message_parts[5].parse::<f32>(),
            ) {
                let mut request_is_neighbour = false;
                for neighbor in &self.routing_table.neighbors {
                    if neighbor.id == requester_id {
                        request_is_neighbour = true;
                    }
                }

                if request_is_neighbour {
                    println!("Requester is not a neighbour: {}", requester_id);
                    return;
                }

                let target = Coordinate {
                    x: target_x,
                    y: target_y,
                };

                if self.is_within_communication_radius(requester_x, requester_y) {
                    self.move_towards(&target);
                } else {
                    // Relay move request to nearest neighbor
                    let nearest_neighbor = self.find_nearest_neighbor();
                    if let Some(neighbor) = nearest_neighbor {
                        // Send message to neighbor requesting it to move towards the target
                        self.send_move_request(&neighbor, &target);
                    } else {
                        println!("No neighbors found to relay move request.");
                    }
                }
            }
        } else {
            println!("Invalid move request format.");
        }
    }

    fn is_within_communication_radius(&self, x: f32, y: f32) -> bool {
        let distance = ((x - self.position.x).powi(2) + (y - self.position.y).powi(2)).sqrt();
        distance <= self.communication_radius
    }

    fn receive_target_from_simulator(&self, message: &str) -> Option<Coordinate> {
        if message == "stop" {
            println!("Stopped loop");
            return None;
        }

        let target = serde_json::from_str(message).ok();
        println!("Received target from simulator: {:?}", target);
        target
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

    fn send_messages(&self, sender:usize,  message: String) {
        for neighbor in &self.routing_table.neighbors {
            if neighbor.id != sender {
                self.send_message(&message, neighbor.id);
            }
        }
    }

    fn send_message(&self, message: &str, to: usize) {
        let message = format!("MESSAGE {} {}", self.id, message);
        let neighbor_address: SocketAddr = format!("127.0.0.1:808{}", to).parse().unwrap();

        self.socket.send_to(message.as_bytes(), neighbor_address)
            .unwrap();

        println!("Sent message to neighbor {}: {}",to, message);
    }

    fn add_neighbour(&mut self, message: &str) {
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
                println!("Neighbour {} added", id);
                self.routing_table.neighbors.push(Neighbor {id, position });
            } else {
                println!("Neighbour to far away.");
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
                } else if message_parts[0] == "ADD_NEIGHBOUR" {
                    self.add_neighbour(message)
                } else if message_parts[0] == "MESSAGE" {
                    self.receive_and_send_message(message)
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
        self.send_finished_to_simulator()
    }
}