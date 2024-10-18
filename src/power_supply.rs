use crate::motherboard::Motherboard;

pub struct PowerSupply {
    motherboard: Motherboard,
}

impl PowerSupply {
    pub fn new(motherboard: Motherboard) -> Self {
        PowerSupply { motherboard }
    }

    pub fn power_on(&mut self) {
        println!("Powering on the system...");
        self.motherboard.power_on();
    }
}