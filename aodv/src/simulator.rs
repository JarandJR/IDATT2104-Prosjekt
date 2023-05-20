#[derive(Clone)]
pub struct Simulator {
    pub drones: u32
}

impl Simulator {
    pub fn new(drones:u32) -> Self {
        Self {
            drones
        }
    }
}
