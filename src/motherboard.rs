use crate::cpu::CPU;
use crate::ram::RAM;
use crate::bios::BIOS;

pub struct Motherboard {
    cpu: CPU,
}

impl Motherboard {
    pub fn new(cpu: CPU) -> Self {
        Motherboard { cpu }
    }

    pub fn power_on(&mut self) {
        println!("System Powered On");
        let mut bios = BIOS::new(std::mem::replace(&mut self.cpu, CPU::new(RAM::new())));
        bios.prompt();
        self.cpu = bios.cpu;
    }
}
