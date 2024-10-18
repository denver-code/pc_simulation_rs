mod ram;
pub mod logic_gates;
mod bios;
mod motherboard;
mod power_supply;
mod utils;
pub mod cpu;

use crate::ram::RAM;
use crate::motherboard::Motherboard;
use crate::power_supply::PowerSupply;
use crate::cpu::CPU;

fn main() {
    let ram = RAM::new();
    let cpu = CPU::new(ram);
    let motherboard = Motherboard::new(cpu);
    let mut power_supply = PowerSupply::new(motherboard);

    power_supply.power_on();
}