pub struct LogicGates;

impl LogicGates {
    pub fn and(a: u8, b: u8) -> u8 {
        a & b
    }

    pub fn or(a: u8, b: u8) -> u8 {
        a | b
    }

    pub fn not(a: u8) -> u8 {
        !a & 0xFF
    }
    pub fn nand(a: u8, b: u8) -> u8 {
        Self::not(Self::and(a, b))
    }

    pub fn nor(a: u8, b: u8) -> u8 {
        Self::not(Self::or(a, b))
    }

    pub fn xor(a: u8, b: u8) -> u8 {
        a ^ b
    }
}
